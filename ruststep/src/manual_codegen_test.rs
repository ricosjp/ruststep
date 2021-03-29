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

use crate::{
    error::*,
    parser::{value::PlaceHolder, Record},
};
use serde::Deserialize;

#[derive(Debug)]
pub struct Ap000 {}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct A {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AHolder {
    pub x: PlaceHolder<f64>,
    pub y: PlaceHolder<f64>,
}

impl AHolder {
    pub fn from_record(record: &Record) -> Result<Self> {
        Ok(Deserialize::deserialize(record)?)
    }

    pub fn into_owned(self) -> A {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::exchange;
    use nom::Finish;

    #[test]
    fn a_from_record() {
        let (_, record) = exchange::simple_record("A_HOLDER(1.0, 2.0)")
            .finish()
            .unwrap();
        let a = AHolder::from_record(dbg!(&record)).unwrap();
        dbg!(a);
    }
}
