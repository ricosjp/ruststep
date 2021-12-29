use ruststep_derive::{as_holder, Holder};
use std::collections::HashMap;

pub struct Table {
    e: HashMap<u64, as_holder!(E)>,
    a: HashMap<u64, as_holder!(A)>,
    b: HashMap<u64, as_holder!(B)>,
}

#[derive(Debug, Clone, PartialEq, ::serde::Deserialize)]
pub struct Simple(pub f64);

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = Table)]
#[holder(field = e)]
#[holder(generate_deserialize)]
pub struct E {
    simple: Simple,
}

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = Table)]
#[holder(field = a)]
#[holder(generate_deserialize)]
pub struct A(#[holder(use_place_holder)] pub E);

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = Table)]
#[holder(field = b)]
#[holder(generate_deserialize)]
pub struct B(pub f64, #[holder(use_place_holder)] pub A);

fn main() {}
