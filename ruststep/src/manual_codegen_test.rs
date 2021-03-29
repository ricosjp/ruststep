//! Experimental schema definitions corresponding following EXPRESS Schema
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
//! END_SCHEMA;
//! ```

use crate::parser::value::{PlaceHolder, RValue};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Ap000 {
    a: HashMap<u64 /* entity id */, A>,
    b: HashMap<u64 /* entity id */, BHolder>,
    c: HashMap<u64 /* entity id */, CHolder>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct A {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct B {
    pub z: f64,
    pub a: A,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename = "B")]
pub struct BHolder {
    pub z: f64,
    pub a: PlaceHolder<A>,
}

impl BHolder {
    pub fn into_owned(self, tables: &Ap000) -> B {
        let BHolder { z, a } = self;
        let a = match a {
            PlaceHolder::Ref(id) => match id {
                RValue::Entity(id) => tables.a[&id].clone(),
                _ => unreachable!("Invalid STEP record"),
            },
            PlaceHolder::Owned(a) => a,
        };
        B { z, a }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct C {
    pub p: A,
    pub q: B,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename = "C")]
pub struct CHolder {
    pub p: PlaceHolder<A>,
    pub q: PlaceHolder<BHolder>,
}

impl CHolder {
    pub fn into_owned(self, tables: &Ap000) -> C {
        let CHolder { p, q } = self;

        let p = match p {
            PlaceHolder::Ref(id) => match id {
                RValue::Entity(id) => tables.a[&id].clone(),
                _ => unreachable!("Invalid STEP record"),
            },
            PlaceHolder::Owned(value) => value,
        };

        let q = match q {
            PlaceHolder::Ref(id) => match id {
                RValue::Entity(id) => tables.b[&id].clone(),
                _ => unreachable!("Invalid STEP record"),
            },
            PlaceHolder::Owned(value) => value,
        }
        .into_owned(tables);

        C { p, q }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::exchange;
    use nom::Finish;

    #[test]
    fn a_from_record() {
        let (_, record) = exchange::simple_record("A(1.0, 2.0)").finish().unwrap();
        let a = A::deserialize(&record).unwrap();
        dbg!(a);
    }

    #[test]
    fn b_from_record() {
        let mut tables = Ap000::default();
        tables.a.insert(2, A { x: 2.0, y: 3.0 });

        let (_, record) = exchange::simple_record("B(1.0, A((2.0, 3.0)))")
            .finish()
            .unwrap();
        let b = BHolder::deserialize(&record).unwrap();
        dbg!(b.into_owned(&tables));

        let (_, record) = exchange::simple_record("B(1.0, #2)").finish().unwrap();
        let b = BHolder::deserialize(&record).unwrap();
        dbg!(b.into_owned(&tables));
    }

    #[test]
    fn c_from_record() {
        // #2 = A((1.0, 2.0))
        // #4 = B((2.0, A((4.0, 5.0))));
        // #5 = B((2.0, #2));
        let mut tables = Ap000::default();
        tables.a.insert(2, A { x: 2.0, y: 3.0 });
        tables.b.insert(
            4,
            BHolder {
                z: 2.0,
                a: PlaceHolder::Owned(A { x: 4.0, y: 5.0 }),
            },
        );
        tables.b.insert(
            5,
            BHolder {
                z: 2.0,
                a: PlaceHolder::Ref(RValue::Entity(2)),
            },
        );

        // All components are inline
        let (_, record) = exchange::simple_record("C(A((1.0, 2.0)), B((1.0, A((2.0, 3.0)))))")
            .finish()
            .unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables));

        // Use B with inline A
        let (_, record) = exchange::simple_record("C(A((1.0, 2.0)), #4)")
            .finish()
            .unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables));

        // Use B with ref A
        let (_, record) = exchange::simple_record("C(A((1.0, 2.0)), #5)")
            .finish()
            .unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables));

        // Use both reference
        let (_, record) = exchange::simple_record("C(#2, #4)").finish().unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables));

        // Use both reference with DAG
        let (_, record) = exchange::simple_record("C(#2, #5)").finish().unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables));
    }
}
