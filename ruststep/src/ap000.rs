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
//!     SUPERTYPE OF (sub1, sub2)
//!     a: f64;
//!   END_ENTITY;
//!
//!   ENTITY sub1;
//!     SUBTYPE OF (base);
//!     b: f64;
//!   END_ENTITY;
//!
//!   ENTITY sub2;
//!     SUBTYPE OF (base);
//!     c: f64;
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
//! let sub = Sub1 { base, b: 1.0 };
//!
//! let mut any: BaseAny = sub.into();
//!
//! // `a` of `Base` is accessible through Deref/DerefMut
//! println!("{}", any.a); // 1.0
//! any.a = 2.0;
//!
//! // downcast to Sub1.
//! let sub = any.as_sub1().unwrap();
//! ```

use crate::{
    ast::{DataSection, EntityInstance},
    error::*,
    tables::*,
};
use derive_more::{Deref, DerefMut};
use ruststep_derive::{as_holder, Holder};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

#[cfg(doc)]
use crate::tables;

/// Tables including entities `A`, `B`, and `C` as their holders.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ap000 {
    a: HashMap<u64, as_holder!(A)>,
    b: HashMap<u64, as_holder!(B)>,
    c: HashMap<u64, as_holder!(C)>,
    base: HashMap<u64, BaseHolder>,
    sub1: HashMap<u64, Sub1Holder>,
    sub2: HashMap<u64, Sub2Holder>,
}

impl Ap000 {
    pub fn from_section(sec: &DataSection) -> Result<Self> {
        let mut table = Ap000::default();

        for entity in &sec.entities {
            match entity {
                EntityInstance::Simple { id, record } => {
                    if !match record.name.as_str() {
                        "A" => table.a.insert(*id, AHolder::deserialize(record)?).is_none(),
                        "B" => table.b.insert(*id, BHolder::deserialize(record)?).is_none(),
                        "C" => table.c.insert(*id, CHolder::deserialize(record)?).is_none(),
                        name @ _ => {
                            return Err(Error::UnknownEntityName {
                                entity_name: name.to_string(),
                                schema: "ap000".to_string(),
                            })
                        }
                    } {
                        return Err(Error::DuplicatedEntity(*id));
                    }
                }
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

impl EntityTable<BaseHolder> for Ap000 {
    fn get_owned(&self, entity_id: u64) -> Result<Base> {
        crate::tables::get_owned(self, &self.base, entity_id)
    }
    fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Base>> + 'table> {
        crate::tables::owned_iter(self, &self.base)
    }
}

impl EntityTable<Sub1Holder> for Ap000 {
    fn get_owned(&self, entity_id: u64) -> Result<Sub1> {
        crate::tables::get_owned(self, &self.sub1, entity_id)
    }
    fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Sub1>> + 'table> {
        crate::tables::owned_iter(self, &self.sub1)
    }
}

impl EntityTable<Sub2Holder> for Ap000 {
    fn get_owned(&self, entity_id: u64) -> Result<Sub2> {
        crate::tables::get_owned(self, &self.sub2, entity_id)
    }
    fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<Sub2>> + 'table> {
        crate::tables::owned_iter(self, &self.sub2)
    }
}

impl EntityTable<BaseAnyHolder> for Ap000 {
    fn get_owned(&self, entity_id: u64) -> Result<BaseAny> {
        if let Ok(owned) = crate::tables::get_owned(self, &self.sub1, entity_id) {
            return Ok(BaseAny::Sub1(owned));
        }
        if let Ok(owned) = crate::tables::get_owned(self, &self.sub2, entity_id) {
            return Ok(BaseAny::Sub2(owned));
        }
        Err(crate::error::Error::UnknownEntity(entity_id))
    }
    fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<BaseAny>> + 'table> {
        let sub1 =
            crate::tables::owned_iter(self, &self.sub1).map(|owned| owned.map(BaseAny::Sub1));
        let sub2 =
            crate::tables::owned_iter(self, &self.sub2).map(|owned| owned.map(BaseAny::Sub2));
        Box::new(sub1.chain(sub2))
    }
}

/// Corresponds to `ENTITY a`
#[derive(Debug, Clone, PartialEq, Serialize, Holder)]
#[holder(table = Ap000, field = a)]
pub struct A {
    pub x: f64,
    pub y: f64,
}

/// Corresponds to `ENTITY b`
#[derive(Debug, Clone, PartialEq, Serialize, Holder)]
#[holder(table = Ap000, field = b)]
pub struct B {
    pub z: f64,
    #[holder(use_place_holder)]
    pub a: A,
}

/// Corresponds to `ENTITY c`
#[derive(Debug, Clone, PartialEq, Serialize, Holder)]
#[holder(table = Ap000, field = c)]
pub struct C {
    #[holder(use_place_holder)]
    pub p: A,
    #[holder(use_place_holder)]
    pub q: B,
}

pub trait SuperTypeAny: ::std::ops::Deref<Target = Self::SuperType> + ::std::ops::DerefMut {
    type SuperType;
}

/// Enum-based supertype implementation for
///
/// ```text
/// ENTITY base;
///   SUPERTYPE OF (sub1, sub2)
///   a: f64;
/// END_ENTITY;
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum BaseAny {
    Sub1(Box<Sub1>),
    Sub2(Box<Sub2>),
}

impl BaseAny {
    pub fn as_sub1(self) -> ::std::result::Result<Sub1, Self> {
        match self {
            BaseAny::Sub1(sub) => Ok(*sub),
            _ => Err(self),
        }
    }
    pub fn as_sub2(self) -> ::std::result::Result<Sub2, Self> {
        match self {
            BaseAny::Sub2(sub) => Ok(*sub),
            _ => Err(self),
        }
    }
}

impl ::std::ops::Deref for BaseAny {
    type Target = Base;
    fn deref(&self) -> &Base {
        match self {
            BaseAny::Sub1(sub) => sub.deref(),
            BaseAny::Sub2(sub) => sub.deref(),
        }
    }
}

impl ::std::ops::DerefMut for BaseAny {
    fn deref_mut(&mut self) -> &mut Base {
        match self {
            BaseAny::Sub1(sub) => sub.deref_mut(),
            BaseAny::Sub2(sub) => sub.deref_mut(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
enum BaseAnyHolder {
    Sub1(Sub1Holder),
    Sub2(Sub2Holder),
}

impl Holder for BaseAnyHolder {
    type Table = Ap000;
    type Owned = BaseAny;
    fn into_owned(self, table: &Self::Table) -> Result<Self::Owned> {
        Ok(match self {
            BaseAnyHolder::Sub1(holder) => BaseAny::Sub1(holder.into_owned(table)?),
            BaseAnyHolder::Sub2(holder) => BaseAny::Sub2(holder.into_owned(table)?),
        })
    }
}

/// Raw state of
///
/// ```text
/// ENTITY base;
///   SUPERTYPE OF (sub1, sub2)
///   a: f64;
/// END_ENTITY;
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Base {
    pub a: f64,
}

/// PlaceHolder for
///
/// ```text
/// ENTITY base;
///   SUPERTYPE OF (sub1, sub2)
///   a: f64;
/// END_ENTITY;
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BaseHolder {
    pub a: f64,
}

impl Holder for BaseHolder {
    type Table = Ap000;
    type Owned = Base;
    fn into_owned(self, _tables: &Ap000) -> Result<Base> {
        let BaseHolder { a } = self;
        Ok(Base { a })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deref, DerefMut)]
pub struct Sub1 {
    #[deref]
    #[deref_mut]
    pub base: Base,
    pub b: f64,
}

impl Into<BaseAny> for Sub1 {
    fn into(self) -> BaseAny {
        BaseAny::Sub1(Box::new(self))
    }
}

#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Deserialize)]
pub struct Sub1Holder {
    base: PlaceHolder<BaseHolder>,
    b: f64,
}

impl Holder for Sub1Holder {
    type Table = Ap000;
    type Owned = Sub1;
    fn into_owned(self, table: &Ap000) -> Result<Self::Owned> {
        let Sub1Holder { base, b } = self;
        Ok(Sub1 {
            base: base.into_owned(table)?,
            b,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deref, DerefMut)]
pub struct Sub2 {
    #[deref]
    #[deref_mut]
    pub base: Base,
    pub c: f64,
}

impl Into<BaseAny> for Sub2 {
    fn into(self) -> BaseAny {
        BaseAny::Sub2(Box::new(self))
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Sub2Holder {
    base: PlaceHolder<BaseHolder>,
    c: f64,
}

impl Holder for Sub2Holder {
    type Table = Ap000;
    type Owned = Sub2;
    fn into_owned(self, table: &Ap000) -> Result<Self::Owned> {
        let Sub2Holder { base, c } = self;
        Ok(Sub2 {
            base: base.into_owned(table)?,
            c,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct User {
    pub data: BaseAny,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct UserHolder {
    data: PlaceHolder<BaseAnyHolder>,
}

impl Holder for UserHolder {
    type Table = Ap000;
    type Owned = User;
    fn into_owned(self, table: &Self::Table) -> Result<Self::Owned> {
        let UserHolder { data } = self;
        Ok(User {
            data: data.into_owned(table)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::*, parser::exchange, place_holder::*};
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
