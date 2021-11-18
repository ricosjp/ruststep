use ruststep_derive::{as_holder, Holder};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
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
}

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = Table)]
#[holder(field = b)]
#[holder(generate_deserialize)]
pub struct B {
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = Table)]
#[holder(generate_deserialize)]
pub enum S1 {
    #[holder(field = a)]
    #[holder(use_place_holder)]
    A(Box<A>),
    #[holder(field = b)]
    #[holder(use_place_holder)]
    B(Box<B>),
}

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = Table)]
#[holder(generate_deserialize)]
pub enum S2 {
    #[holder(field = a)]
    #[holder(use_place_holder)]
    A(Box<A>),
    // mix primitive type
    P(f64),
}

fn main() {}
