use ruststep_derive::{as_holder, Holder};
use std::collections::HashMap;

pub struct Table {
    a: HashMap<u64, as_holder!(A)>,
    b: HashMap<u64, as_holder!(B)>,
    c: HashMap<u64, as_holder!(C)>,
}

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = Table)]
#[holder(field = a)]
pub struct A {
    pub x: f64,
    pub y: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = Table)]
#[holder(field = b)]
pub struct B {
    pub z: f64,
    #[holder(use_place_holder)]
    pub a: A,
}

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = Table)]
#[holder(field = c)]
pub struct C {
    #[holder(use_place_holder)]
    pub p: A,
    #[holder(use_place_holder)]
    pub q: Option<B>,
}

fn main() {}
