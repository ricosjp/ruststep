use nom::Finish;
use ruststep::{ast::*, parser::exchange, place_holder::*};
use ruststep_derive::as_holder;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Table {
    a: HashMap<u64, as_holder!(A)>,
    b: HashMap<u64, as_holder!(B)>,
}

#[derive(Clone, Debug, PartialEq, ruststep_derive::Holder)]
#[holder(table = Table)]
#[holder(field = a)]
#[holder(generate_deserialize)]
pub struct A {
    x: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, ruststep_derive::Holder)]
#[holder(table = Table)]
#[holder(field = b)]
#[holder(generate_deserialize)]
pub struct B {
    #[holder(use_place_holder)]
    a: Vec<A>,
}

#[test]
fn deserialize_list_a() {
    let (residual, p): (_, Record) = exchange::simple_record("A((1.0, 2.0))").finish().unwrap();
    dbg!(&p);
    assert_eq!(residual, "");
    let a: AHolder = Deserialize::deserialize(&p).unwrap();
    dbg!(&a);
    assert_eq!(a, AHolder { x: vec![1.0, 2.0] });
}

#[test]
fn deserialize_list_b() {
    let (residual, p): (_, Record) = exchange::simple_record("B((A(((1.0)))))").finish().unwrap();
    dbg!(&p);
    assert_eq!(residual, "");
    let b: BHolder = Deserialize::deserialize(&p).unwrap();
    dbg!(&b);
    assert_eq!(
        b,
        BHolder {
            a: vec![PlaceHolder::Owned(AHolder { x: vec![1.0] })]
        }
    );
}
