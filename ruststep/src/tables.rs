//! Traits for espr-generated structures

use serde::de;
use std::{collections::HashMap, marker::PhantomData};

use crate::{
    ast::{DataSection, Record},
    error::*,
};

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

pub struct StringVisitor {}

impl<'de> de::Visitor<'de> for StringVisitor {
    type Value = String;
    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "String")
    }

    fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        if let Some(s) = seq.next_element()? {
            Ok(s)
        } else {
            panic!("Empty sequence")
        }
    }

    fn visit_str<E>(self, v: &str) -> ::std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }
}

impl WithVisitor for String {
    type Visitor = StringVisitor;
    fn visitor_new() -> Self::Visitor {
        StringVisitor {}
    }
}

pub struct FloatVisitor {}

impl<'de> de::Visitor<'de> for FloatVisitor {
    type Value = f64;
    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "f64")
    }

    fn visit_f64<E>(self, v: f64) -> ::std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v)
    }

    fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: ::serde::de::SeqAccess<'de>,
    {
        Ok(seq.next_element()?.expect("Empty sequence"))
    }
}

impl WithVisitor for f64 {
    type Visitor = FloatVisitor;
    fn visitor_new() -> Self::Visitor {
        FloatVisitor {}
    }
}

pub struct ListVisitor<T: WithVisitor> {
    phantom: PhantomData<T>,
}

impl<'de, T: WithVisitor> de::Visitor<'de> for ListVisitor<T> {
    type Value = Vec<T>;
    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "Vec<{}>", std::any::type_name::<T>())
    }

    fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: ::serde::de::SeqAccess<'de>,
    {
        let n = seq.size_hint().expect("seq must have size hint");
        let mut out = Vec::new();
        for _ in 0..n {
            let visitor = T::visitor_new();
            out.push(visitor.visit_seq(&mut seq)?);
        }
        Ok(out)
    }
}

impl<T: WithVisitor> WithVisitor for Vec<T> {
    type Visitor = ListVisitor<T>;
    fn visitor_new() -> Self::Visitor {
        ListVisitor {
            phantom: PhantomData,
        }
    }
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
    if let Some(_) = table.insert(id, de::Deserialize::deserialize(record)?) {
        Err(Error::DuplicatedEntity(id))
    } else {
        Ok(())
    }
}
