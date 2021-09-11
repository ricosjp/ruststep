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
pub struct A {
    pub x: f64,
}

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = Table)]
#[holder(field = b)]
pub struct B {
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = Table)]
pub enum S {
    #[holder(field = a)]
    A(Box<A>),
    #[holder(field = b)]
    B(Box<B>),
}

fn main() {}
