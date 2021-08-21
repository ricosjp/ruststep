use ruststep_derive::as_holder;
use std::collections::HashMap;

pub struct Table {
    base: HashMap<u64, as_holder!(Base)>,
    sub1: HashMap<u64, as_holder!(Sub1)>,
    sub2: HashMap<u64, as_holder!(Sub2)>,
}

#[derive(Clone, Debug, PartialEq, ruststep_derive::Holder)]
#[holder(table = Table, field = base)]
pub struct Base {
    x: f64,
}

#[derive(Clone, Debug, PartialEq, ruststep_derive::Holder)]
#[holder(table = Table, field = sub1)]
pub struct Sub1 {
    base: Base,
    y1: f64,
}

#[derive(Clone, Debug, PartialEq, ruststep_derive::Holder)]
#[holder(table = Table, field = sub2)]
pub struct Sub2 {
    base: Base,
    y2: f64,
}

#[derive(Clone, Debug, PartialEq)]
enum BaseAny {
    Sub1(Sub1),
    Sub2(Sub2),
}
