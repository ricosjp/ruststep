//! Handling "exchange structure graph" as tables
//!
//! Since records in an exchange structure has references to other records,
//! then consists a graph.
//!
//! - An exchange structure corresponds to a graph, we call it "exchange structure graph" here.
//! - A node of graph corresponds to a record.
//! - An edge of graph corresponds to a reference in a record.
//!
//! Creating table from exchange structure AST
//! -------------------------------------------
//! Let us consider a simple EXPRESS schema:
//!
//! ```text
//! ENTITY a;
//!   x: INTEGER;
//!   y: INTEGER;
//! END_ENTITY;
//!
//! ENTITY b;
//!   z: INTEGER;
//!   w: a;
//! END_ENTITY;
//! ```
//!
//! Corresponding data section in STEP file will be something like following (skip HEADER section):
//!
//! ```text
//! DATA;
//!   #1 = A(1, 2);
//!   #2 = A(3, 4);
//!   #3 = B(5, #1);
//!   #4 = B(6, #1);
//!   #5 = B(7, #2);
//!   #6 = B(8, A((9, 10)));
//! ENDSEC;
//! ```
//!
//! In this example, `#3` and `#4` has reference to `#1`.
//! There will exist non-exclusive reference between entity instances generally,
//! and thus the data must be regarded as a graph.
//!
//! ruststep will parse this data section into following tables:
//!
//! | Table (a) | x (i64) | y (i64) |
//! |:----------|:--------|:--------|
//! | `#1`      | 1       | 2       |
//! | `#2`      | 3       | 4       |
//!
//! | Table (b) | z (i64) | w (a) |
//! |:----------|:--------|:------|
//! | `#3`      | 5       | `#1`  |
//! | `#4`      | 6       | `#1`  |
//! | `#5`      | 7       | `#2`  |
//! | `#6`      | 8       | `A((9, 10))` |
//!
//! Each columns are defined by EXPRESS schema.
//! `x`, `y`, and `z` are specified as integer in EXPRESS, and will be treated as `i64` in Rust code.
//! The simple types in EXPRESS are mapped into Rust primitive types.
//! The ENTITY `a` will be treated as a Rust struct like
//!
//! ```
//! struct A {
//!   x: i64,
//!   y: i64,
//! }
//! ```
//!
//! The ENTITY `b` has to support both reference and inline struct like as `#4` and `#6`.
//! For this purpose, [PlaceHolder] exists:
//!
//! ```
//! # use ruststep::ast::Name;
//! enum PlaceHolder<T> {
//!   /// For reference, e.g. `#1`
//!   Ref(Name),
//!   /// For inline typed parameter, e.g. `A((9, 10))`
//!   Owned(T),
//! }
//! ```
//!
//! Then following two Rust structs will be defined:
//!
//! ```
//! # use ruststep::tables::PlaceHolder;
//! # struct A {}
//! # struct AHolder {}
//! struct B {
//!   z: i64,
//!   w: A,
//! }
//! struct BHolder {
//!   z: i64,
//!   w: PlaceHolder<AHolder>,
//! }
//! ```
//!
//! There also a function [IntoOwned::into_owned] to convert a holder struct
//! `BHolder` into owned struct `B`.
//! `AHolder` will also be introduced to keep consistency.
//! These are automated by [ruststep_derive::Holder] proc-macro.
//!

use crate::{ast::*, error::*};
use serde::{
    de::{self, IntoDeserializer, VariantAccess},
    Deserialize,
};
use std::{collections::HashMap, fmt, marker::PhantomData};

/// Trait for resolving a reference through entity id
pub trait IntoOwned: Clone + 'static {
    type Owned;
    type Table;
    fn into_owned(self, table: &Self::Table) -> Result<Self::Owned>;
}

impl<T: IntoOwned> IntoOwned for Vec<T> {
    type Owned = Vec<T::Owned>;
    type Table = T::Table;
    fn into_owned(self, table: &Self::Table) -> Result<Self::Owned> {
        self.into_iter().map(|x| x.into_owned(table)).collect()
    }
}

/// Trait for a field of tables
pub trait Holder: IntoOwned {
    fn name() -> &'static str;
    fn attr_len() -> usize;
}

pub trait WithVisitor {
    type Visitor: for<'de> de::Visitor<'de, Value = Self>;
    fn visitor_new() -> Self::Visitor;
}

/// Trait for tables which pulls an entity (`T`) from an entity id (`u64`)
pub trait EntityTable<T: Holder<Table = Self>> {
    /// Get owned entity from table
    fn get_owned(&self, entity_id: u64) -> Result<T::Owned>;

    /// Get owned entities as an iterator
    fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<T::Owned>> + 'table>;
}

/// Create Table from [DataSection]
pub trait TableInit: Default {
    fn append_data_section(&mut self, section: &DataSection) -> Result<()>;

    fn from_data_section(section: &DataSection) -> Result<Self> {
        let mut table = Self::default();
        table.append_data_section(section)?;
        Ok(table)
    }

    fn from_data_sections(sections: &[DataSection]) -> Result<Self> {
        let mut table = Self::default();
        for section in sections {
            table.append_data_section(section)?;
        }
        Ok(table)
    }
}

pub fn get_owned<T, Table>(table: &Table, map: &HashMap<u64, T>, entity_id: u64) -> Result<T::Owned>
where
    T: Holder<Table = Table>,
    Table: EntityTable<T>,
{
    match map.get(&entity_id) {
        Some(holder) => holder.clone().into_owned(table),
        None => Err(Error::UnknownEntity(entity_id)),
    }
}

pub fn owned_iter<'table, T, Table>(
    table: &'table Table,
    map: &'table HashMap<u64, T>,
) -> Box<dyn Iterator<Item = Result<T::Owned>> + 'table>
where
    T: Holder<Table = Table>,
    Table: EntityTable<T>,
{
    Box::new(
        map.values()
            .cloned()
            .map(move |value| value.into_owned(table)),
    )
}

/// Helper function to implement TableInit trait
pub fn insert_record<'de, T: de::Deserialize<'de>>(
    table: &mut HashMap<u64, T>,
    id: u64,
    record: &Record,
) -> crate::error::Result<()> {
    if table
        .insert(id, de::Deserialize::deserialize(record)?)
        .is_some()
    {
        Err(Error::DuplicatedEntity(id))
    } else {
        Ok(())
    }
}

/// Owned value or reference through entity/value id
#[derive(Debug, Clone, PartialEq)]
pub enum PlaceHolder<T> {
    Ref(Name),
    Owned(T),
}

impl<T: Holder> IntoOwned for PlaceHolder<T>
where
    T::Table: EntityTable<T>,
{
    type Owned = T::Owned;
    type Table = T::Table;
    /// Get owned value, or look up entity table and clone it for a reference.
    ///
    /// Errors
    /// -------
    /// - if table lookup failed, i.e. unknown entity id not registered in the table
    ///
    fn into_owned(self, table: &Self::Table) -> Result<T::Owned> {
        match self {
            PlaceHolder::Ref(id) => match id {
                Name::Entity(id) => table.get_owned(id),
                _ => unimplemented!("ENTITY is only supported now"),
            },
            PlaceHolder::Owned(a) => a.into_owned(table),
        }
    }
}

impl<T: Holder> From<T> for PlaceHolder<T> {
    fn from(owned: T) -> Self {
        PlaceHolder::Owned(owned)
    }
}

impl<T> From<Name> for PlaceHolder<T> {
    fn from(rvalue: Name) -> Self {
        PlaceHolder::Ref(rvalue)
    }
}

impl<'de, T: Holder + WithVisitor + Deserialize<'de>> Deserialize<'de> for PlaceHolder<T> {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple_struct(
            T::name(),
            T::attr_len(),
            PlaceHolderVisitor::<T>::default(),
        )
    }
}

struct PlaceHolderVisitor<T> {
    phantom: PhantomData<T>,
}

impl<T> Default for PlaceHolderVisitor<T> {
    fn default() -> Self {
        PlaceHolderVisitor {
            phantom: PhantomData,
        }
    }
}

impl<'de, T: Deserialize<'de> + Holder + WithVisitor> de::Visitor<'de> for PlaceHolderVisitor<T> {
    type Value = PlaceHolder<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "PlaceHolder<{}>", std::any::type_name::<T>())
    }

    fn visit_i64<E>(self, v: i64) -> ::std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(PlaceHolder::Owned(T::deserialize(v.into_deserializer())?))
    }

    fn visit_f64<E>(self, v: f64) -> ::std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(PlaceHolder::Owned(T::deserialize(v.into_deserializer())?))
    }

    fn visit_str<E>(self, v: &str) -> ::std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(PlaceHolder::Owned(T::deserialize(v.into_deserializer())?))
    }

    fn visit_seq<A>(self, seq: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let visitor = T::visitor_new();
        Ok(PlaceHolder::Owned(visitor.visit_seq(seq)?))
    }

    // For Ref(Name)
    fn visit_enum<A>(self, data: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: de::EnumAccess<'de>,
    {
        let (key, variant): (String, _) = data.variant()?;
        match key.as_str() {
            "Entity" => {
                let value: u64 = variant.newtype_variant()?;
                Ok(PlaceHolder::Ref(Name::Entity(value)))
            }
            "Value" => {
                let value: u64 = variant.newtype_variant()?;
                Ok(PlaceHolder::Ref(Name::Value(value)))
            }
            "ConstantEntity" => {
                let name: String = variant.newtype_variant()?;
                Ok(PlaceHolder::Ref(Name::ConstantEntity(name)))
            }
            "ConstantValue" => {
                let name: String = variant.newtype_variant()?;
                Ok(PlaceHolder::Ref(Name::ConstantValue(name)))
            }
            _ => unreachable!("Invalid key while deserializing PlaceHolder"),
        }
    }

    // Entry point for Record or Parameter::Typed
    fn visit_map<A>(self, map: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let visitor = T::visitor_new();
        Ok(PlaceHolder::Owned(visitor.visit_map(map)?))
    }
}
