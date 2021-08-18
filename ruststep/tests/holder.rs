use ruststep_derive::as_holder;
use std::collections::HashMap;

pub struct Table {
    a: HashMap<u64, as_holder!(A)>,
}

#[derive(Debug, Clone, ruststep_derive::Holder)]
#[holder(table = Table, field = a)]
pub struct A {
    x: f64,
    y: f64,
}
