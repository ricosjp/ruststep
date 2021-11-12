// Test for deserializing Holder structs

use ruststep::{ast::*, error::*, parser::exchange, place_holder::PlaceHolder, tables::*};
use ruststep_derive::{as_holder, Holder, TableInit};

use nom::Finish;
use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};

#[derive(TableInit, Default)]
pub struct Table {
    a: HashMap<u64, as_holder!(A)>,
    b: HashMap<u64, as_holder!(B)>,
}

impl FromStr for Table {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        let data_sec = DataSection::from_str(input)?;
        Ok(Self::from_data_section(&data_sec)?)
    }
}

impl Table {
    fn example() -> Self {
        Self::from_str(
            r#"
            DATA;
              #1 = A(1.0, 2.0);
              #2 = B(3.0, A((4.0, 5.0)));
              #3 = B(6.0, #1);
            ENDSEC;
            "#,
        )
        .unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = Table)]
#[holder(field = a)]
#[holder(generate_deserialize)]
pub struct A {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = Table)]
#[holder(field = b)]
#[holder(generate_deserialize)]
pub struct B {
    pub z: f64,
    #[holder(use_place_holder)]
    pub a: A,
}

#[test]
fn deserialize_a_holder() {
    // from Record
    let (residual, p): (_, Record) = exchange::simple_record("A(1.0, 2.0)").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: AHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(a, AHolder { x: 1.0, y: 2.0 });

    // from Parameter::Typed
    let (residual, p): (_, Parameter) = exchange::parameter("A((1.0, 2.0))").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: AHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(a, AHolder { x: 1.0, y: 2.0 });

    // Test for PlaceHolder<AHolder>
    let (residual, p): (_, Record) = exchange::simple_record("A(1.0, 2.0)").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: PlaceHolder<AHolder> = Deserialize::deserialize(&p).unwrap();
    assert_eq!(a, PlaceHolder::Owned(AHolder { x: 1.0, y: 2.0 }));
}

#[test]
fn deserialize_b_holder_record() {
    // from Record
    let (residual, p): (_, Record) = exchange::simple_record("B(1.0, A((2.0, 3.0)))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: BHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        b,
        BHolder {
            z: 1.0,
            a: PlaceHolder::Owned(AHolder { x: 2.0, y: 3.0 })
        }
    );
}

#[test]
fn deserialize_b_holder_record_ref() {
    // from Record with ref
    let (residual, p): (_, Record) = exchange::simple_record("B(1.0, #2)").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: BHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        b,
        BHolder {
            z: 1.0,
            a: PlaceHolder::Ref(RValue::Entity(2))
        }
    );
}

#[test]
fn deserialize_b_holder_parameter() {
    // from Parameter::Typed
    let (residual, p): (_, Parameter) = exchange::parameter("B((1.0, A((2.0, 3.0))))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: BHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        b,
        BHolder {
            z: 1.0,
            a: PlaceHolder::Owned(AHolder { x: 2.0, y: 3.0 })
        }
    );
}

#[test]
fn deserialize_b_holder_parameter_ref() {
    // from Parameter::Typed with Ref
    let (residual, p): (_, Parameter) = exchange::parameter("B((1.0, #2))").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: BHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        b,
        BHolder {
            z: 1.0,
            a: PlaceHolder::Ref(RValue::Entity(2))
        }
    );
}

#[test]
fn get_owned_a() {
    let table = Table::example();
    let a = EntityTable::<AHolder>::get_owned(&table, 1).unwrap();
    assert_eq!(a, A { x: 1.0, y: 2.0 });
}

#[test]
fn get_owned_b2() {
    let table = Table::example();
    let b = EntityTable::<BHolder>::get_owned(&table, 2).unwrap();
    assert_eq!(
        b,
        B {
            z: 3.0,
            a: A { x: 4.0, y: 5.0 }
        }
    );
}

#[test]
fn get_owned_b3() {
    let table = Table::example();
    let b = EntityTable::<BHolder>::get_owned(&table, 3).unwrap();
    assert_eq!(
        b,
        B {
            z: 6.0,
            a: A { x: 1.0, y: 2.0 }
        }
    );
}
