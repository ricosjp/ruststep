//! Manually generated schema definitions corresponding following EXPRESS Schema
//!
//! ```text
//! SCHEMA ap000;
//!   ENTITY a;
//!     x: REAL;
//!     y: REAL;
//!   END_ENTITY;
//!
//!   ENTITY b;
//!     z: REAL;
//!     w: a;
//!   END_ENTITY;
//!
//!   ENTITY c;
//!     p: a;
//!     q: b;
//!   END_ENTITY;
//!
//!   -- For subtype/supertype
//!   ENTITY base;
//!     SUPERTYPE OF (sub)
//!     a: f64;
//!   END_ENTITY;
//!
//!   ENTITY sub;
//!     SUBTYPE OF (base);
//!     b: f64;
//!   END_ENTITY;
//!
//!   ENTITY user;
//!     data: base;
//!   END_ENTITY;
//! END_SCHEMA;
//! ```
//!
//! This sub-module is for help designing and testing generated code.
//! Most functionality in generated codes are supplied as trait in [tables].
//!
//! Examples
//! ---------
//!
//! ```
//! use ruststep::*;
//!
//! const STEP_INPUT: &str = r#"
//! ISO-10303-21;
//! HEADER;
//!   FILE_DESCRIPTION((''), '');
//!   FILE_NAME('ruststep/examples/ap000/read.step', '2018-04-27T08:23:47', (''), (''), '', '', '');
//!   FILE_SCHEMA(('AP000'));
//! ENDSEC;
//! DATA;
//!   #1 = A(1.0, 2.0);
//!   #2 = B(3.0, #1);
//!   #3 = B(3.0, A((4.0, 5.0)));
//!   #4 = C(#1, #2);
//!   #5 = C(#1, #3);
//!   #6 = C(#1, B((6.0, #1)));
//!   #7 = C(#1, B((6.0, A((7.0, 8.0)))));
//!   #8 = C(A((9.0, 10.0)), #2);
//!   #9 = C(A((11.0, 12.0)), #3);
//! ENDSEC;
//! END-ISO-10303-21;
//! "#;
//!
//! // Parse input string into an exchange structure
//! let step = parser::parse(STEP_INPUT.trim()).unwrap();
//!
//! // STEP file can contain multiple DATA section,
//! // and assumes it be 1 here.
//! assert_eq!(step.data.len(), 1);
//!
//! // Load DATA section as tables of each entity
//! let table = ap000::Ap000::from_section(&step.data[0]).unwrap();
//!
//! // Iterate over entity instances
//! for c in table.c_iter() {
//!     let c_owned = c.unwrap(); // Entity reference e.g. `#1` is resolved here.
//!                               // If an undefined entity is contained, `c` will be
//!                               // `ruststep::error::Error::UnknownEntity`
//!     println!("C = {:?}", c_owned);
//! }
//! ```
//!
//! custom `Any` trait for entity `a`
//!
//! ```
//! use ruststep::ap000::*;
//!
//! let base = Base { a: 1.0 };
//! let sub = Sub { base, b: 1.0 };
//!
//! let sub_r = &sub as &dyn BaseAny;
//!
//! // call Debug for Sub by dispatch
//! dbg!(&sub_r);
//!
//! let sub2: &Sub = sub_r.downcast_ref().unwrap();
//! ```

use crate::{
    ast::{DataSection, EntityInstance},
    custom_any,
    error::*,
    place_holder::*,
    tables::*,
};
use serde::{de, Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

#[cfg(doc)]
use crate::tables;

/// Tables including entities `A`, `B`, and `C` as their holders.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ap000 {
    a: HashMap<u64, AHolder>,
    b: HashMap<u64, BHolder>,
    c: HashMap<u64, CHolder>,
}

impl Ap000 {
    pub fn from_section(sec: &DataSection) -> Result<Self> {
        let mut table = Ap000::default();

        for entity in &sec.entities {
            match entity {
                EntityInstance::Simple { name, record } => match record.name.as_str() {
                    "A" => table
                        .a
                        .insert(*name, AHolder::deserialize(record)?)
                        .is_none(),
                    "B" => table
                        .b
                        .insert(*name, BHolder::deserialize(record)?)
                        .is_none(),
                    "C" => table
                        .c
                        .insert(*name, CHolder::deserialize(record)?)
                        .is_none(),
                    _ => panic!(),
                },
                EntityInstance::Complex { .. } => unimplemented!(),
            };
        }
        Ok(table)
    }

    pub fn a_iter<'table>(&'table self) -> impl Iterator<Item = Result<A>> + 'table {
        EntityTable::<AHolder>::owned_iter(self)
    }

    pub fn b_iter<'table>(&'table self) -> impl Iterator<Item = Result<B>> + 'table {
        EntityTable::<BHolder>::owned_iter(self)
    }

    pub fn c_iter<'table>(&'table self) -> impl Iterator<Item = Result<C>> + 'table {
        EntityTable::<CHolder>::owned_iter(self)
    }
}

impl EntityTable<AHolder> for Ap000 {
    fn get_owned(&self, entity_id: u64) -> Result<A> {
        crate::tables::get_owned(self, &self.a, entity_id)
    }
    fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<A>> + 'table> {
        crate::tables::owned_iter(self, &self.a)
    }
}

impl EntityTable<BHolder> for Ap000 {
    fn get_owned(&self, entity_id: u64) -> Result<B> {
        crate::tables::get_owned(self, &self.b, entity_id)
    }
    fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<B>> + 'table> {
        crate::tables::owned_iter(self, &self.b)
    }
}

impl EntityTable<CHolder> for Ap000 {
    fn get_owned(&self, entity_id: u64) -> Result<C> {
        crate::tables::get_owned(self, &self.c, entity_id)
    }
    fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<C>> + 'table> {
        crate::tables::owned_iter(self, &self.c)
    }
}

/// Corresponds to `ENTITY a`
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct A {
    pub x: f64,
    pub y: f64,
}

/// Holder for [A]
#[derive(Debug, Clone, PartialEq)]
pub struct AHolder {
    x: f64,
    y: f64,
}

struct AHolderVisitor;

impl<'de> de::Visitor<'de> for AHolderVisitor {
    type Value = AHolder;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "AHolder")
    }

    fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        if let Some(size) = seq.size_hint() {
            if size != AHolder::attr_len() {
                todo!("Create another error and send it")
            }
        }
        let x = seq.next_element()?.unwrap();
        let y = seq.next_element()?.unwrap();
        Ok(AHolder { x, y })
    }

    // Entry point for Record or Parameter::Typed
    fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let key: String = map
            .next_key()?
            .expect("Empty map cannot be accepted as ruststep Holder"); // this must be a bug, not runtime error
        if key != AHolder::name() {
            todo!("Create Error type and send it")
        }
        let value = map.next_value()?; // send to Self::visit_seq
        Ok(value)
    }
}

impl<'de> de::Deserialize<'de> for AHolder {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple_struct(Self::name(), Self::attr_len(), AHolderVisitor {})
    }
}

impl Holder for AHolder {
    type Table = Ap000;
    type Owned = A;
    fn into_owned(self, _tables: &Ap000) -> Result<A> {
        let AHolder { x, y } = self;
        Ok(A { x, y })
    }
    fn name() -> &'static str {
        "A"
    }
    fn attr_len() -> usize {
        2
    }
}

/// Corresponds to `ENTITY b`
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct B {
    pub z: f64,
    pub a: A,
}

/// Holder for [B]
#[derive(Debug, Clone, PartialEq)]
pub struct BHolder {
    z: f64,
    a: PlaceHolder<AHolder>,
}

impl<'de> de::Deserialize<'de> for BHolder {
    fn deserialize<D>(_deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        todo!()
    }
}

impl Holder for BHolder {
    type Table = Ap000;
    type Owned = B;
    fn into_owned(self, tables: &Ap000) -> Result<B> {
        let BHolder { z, a } = self;
        Ok(B {
            z,
            a: a.into_owned(tables)?,
        })
    }
    fn name() -> &'static str {
        "B"
    }
    fn attr_len() -> usize {
        2
    }
}

/// Corresponds to `ENTITY c`
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct C {
    pub p: A,
    pub q: B,
}

/// Holder for [C]
#[derive(Debug, Clone, PartialEq)]
pub struct CHolder {
    p: PlaceHolder<AHolder>,
    q: PlaceHolder<BHolder>,
}

impl<'de> de::Deserialize<'de> for CHolder {
    fn deserialize<D>(_deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        todo!()
    }
}

impl Holder for CHolder {
    type Table = Ap000;
    type Owned = C;
    fn into_owned(self, tables: &Ap000) -> Result<C> {
        let CHolder { p, q } = self;
        Ok(C {
            p: p.into_owned(tables)?,
            q: q.into_owned(tables)?,
        })
    }
    fn name() -> &'static str {
        "C"
    }
    fn attr_len() -> usize {
        2
    }
}

custom_any!(BaseAny);

#[derive(Debug, Clone)]
pub struct Base {
    pub a: f64,
}
impl BaseAny for Base {}

#[derive(Debug, Clone)]
pub struct Sub {
    pub base: Base,
    pub b: f64,
}
impl BaseAny for Sub {}

#[derive(Debug, Clone)]
pub struct User {
    pub data: Box<dyn BaseAny>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::*, parser::exchange};
    use nom::Finish;

    #[test]
    fn a_from_record() {
        let (_, record) = exchange::simple_record("A(1.0, 2.0)").finish().unwrap();
        let a = AHolder::deserialize(&record).unwrap();
        dbg!(a);
    }

    // Example Tables generated by
    //
    // ```
    // DATA;
    //   #2 = A(1.0, 2.0);
    //   #4 = B(2.0, A((4.0, 5.0)));
    //   #5 = B(2.0, #2);
    // ENDSEC;
    // ```
    fn example_table() -> Ap000 {
        let mut tables = Ap000::default();
        tables.a.insert(2, AHolder { x: 1.0, y: 2.0 });
        tables.b.insert(
            4,
            BHolder {
                z: 2.0,
                a: PlaceHolder::Owned(AHolder { x: 4.0, y: 5.0 }),
            },
        );
        tables.b.insert(
            5,
            BHolder {
                z: 2.0,
                a: PlaceHolder::Ref(RValue::Entity(2)),
            },
        );
        tables
    }

    #[test]
    fn section_to_table() {
        let (_, sec) = exchange::data_section(
            r#"
            DATA;
              #2 = A(1.0, 2.0);
              #4 = B(2.0, A((4.0, 5.0)));
              #5 = B(2.0, #2);
            ENDSEC;
            "#
            .trim(),
        )
        .finish()
        .unwrap();
        dbg!(&sec);

        let table = Ap000::from_section(&sec).unwrap();
        dbg!(&table);
        assert_eq!(table, example_table());
    }

    #[test]
    fn b_from_record() {
        let tables = example_table();

        let (_, record) = exchange::simple_record("B(1.0, A((2.0, 3.0)))")
            .finish()
            .unwrap();
        let b = BHolder::deserialize(&record).unwrap();
        dbg!(b.into_owned(&tables).unwrap());

        let (_, record) = exchange::simple_record("B(1.0, #2)").finish().unwrap();
        let b = BHolder::deserialize(&record).unwrap();
        dbg!(b.into_owned(&tables).unwrap());
    }

    #[test]
    fn c_from_record() {
        let tables = example_table();

        // All components are inline
        let (_, record) = exchange::simple_record("C(A((1.0, 2.0)), B((1.0, A((2.0, 3.0)))))")
            .finish()
            .unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables).unwrap());

        // Use B with inline A
        let (_, record) = exchange::simple_record("C(A((1.0, 2.0)), #4)")
            .finish()
            .unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables).unwrap());

        // Use B with ref A
        let (_, record) = exchange::simple_record("C(A((1.0, 2.0)), #5)")
            .finish()
            .unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables).unwrap());

        // Use both reference
        let (_, record) = exchange::simple_record("C(#2, #4)").finish().unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables).unwrap());

        // Use both reference with DAG
        let (_, record) = exchange::simple_record("C(#2, #5)").finish().unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables).unwrap());

        // Inline struct with reference
        let (_, record) = exchange::simple_record("C(#2, B((6.0, #2)))")
            .finish()
            .unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables).unwrap());
    }
}
