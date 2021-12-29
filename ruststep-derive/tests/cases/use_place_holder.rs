use ruststep_derive::{as_holder, Holder};
use std::collections::HashMap;

pub struct Table {
    a: HashMap<u64, as_holder!(A)>,
    b: HashMap<u64, as_holder!(B)>,
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

fn main() {}
