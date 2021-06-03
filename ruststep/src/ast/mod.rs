//! AST (abstract syntax tree) for [exchange structure (ISO-10303-21)][ISO-10303-21]
//!
//! [ISO-10303-21]: https://www.iso.org/standard/63141.html
//!
//! Deserialize `Parameter` into Rust struct
//! ----------------------------------------
//!
//! [Parameter] and [Record] can be deserialize through serde data model.
//!
//! | Parameter   | serde data model |
//! |:------------|:-----------------|
//! | Integer     | i64              |
//! | Real        | f64              |
//! | String      | string           |
//! | Enumeration | -                |
//! | List        | seq              |
//! | NotProvided | unit             |
//! | Omitted     | unit             |
//! | Typed       | map              |
//! | RValue      | enum             |
//!
//! See [the official document of serde data model](https://serde.rs/data-model.html) for detail.
//!
//! - `Parameter::Typed` is mapped to "map"
//!   e.g. `A((1.0, 2.0))` will be deserialized into `{ "A": [1.0, 2.0] }`.
//! - `Parameter::RValue` is mapped to "map"
//!   e.g. an entity reference `#12` will be deserialized into `{ "Entity": 12 }`.
//! - FIXME: Enumeration is not supported yet.
//!
//! Examples
//! ---------
//!
//! - Untyped parameters, e.g. real number
//!
//! ```
//! use serde::Deserialize;
//! use ruststep::ast::Parameter;
//!
//! #[derive(Debug, Deserialize)]
//! struct A {
//!     x: f64,
//!     y: f64,
//! }
//!
//! // Create a list as `Parameter::List`
//! let p = Parameter::from_iter(&[Parameter::real(1.0), Parameter::real(2.0)]);
//!
//! // Deserialize the `Parameter` sequence into `A` because serde allows upcasting seq to struct
//! let a: A = Deserialize::deserialize(&p).unwrap();
//! println!("{:?}", a);
//!
//! // Input types will be checked at runtime:
//! let p = Parameter::from_iter(&[Parameter::string("a"), Parameter::integer(2)]);
//! let result: Result<A, _> = Deserialize::deserialize(&p);
//! assert!(result.is_err());
//! ```
//!
//! - Typed parameter, e.g. `A(1)`
//!   - FIXME: Type name check is not implemented yet.
//!
//! ```
//! use serde::Deserialize;
//! use ruststep::parser::exchange;
//! use nom::Finish;
//!
//! #[derive(Debug, Deserialize)]
//! struct A {
//!     x: f64,
//!     y: f64,
//! }
//!
//! let (res, p) = exchange::parameter("A((1.0, 2.0))").finish().unwrap();
//! assert_eq!(res, "");
//! let a: A = Deserialize::deserialize(&p).unwrap();
//! dbg!(a);
//! ```
//!
//! - For [RValue]
//!
//! ```
//! use serde::Deserialize;
//! use ruststep::{parser::exchange, ast::RValue};
//! use nom::Finish;
//!
//! let (res, p) = exchange::parameter("#11").finish().unwrap();
//! let a: RValue = Deserialize::deserialize(&p).unwrap();
//! assert_eq!(a, RValue::Entity(11))
//! ```
//!

mod parameter;
mod record;
mod ser;
mod single_map_deserializer;
mod value;

pub use parameter::*;
pub use record::*;
pub use ser::*;
pub use single_map_deserializer::*;
pub use value::*;

/// Entire exchange structure
#[derive(Debug, Clone, PartialEq)]
pub struct Exchange {
    /// `HEADER` section
    pub header: Vec<Record>,
    /// `ANCHOR` section
    pub anchor: Vec<Anchor>,
    /// `REFERENCE` section
    pub reference: Vec<ReferenceEntry>,
    /// `DATA` section
    pub data: Vec<DataSection>,
    /// `SIGNATURE` section
    pub signature: Vec<String>,
}

/// `DATA` section in STEP file
#[derive(Debug, Clone, PartialEq)]
pub struct DataSection {
    /// Metadata
    pub meta: Vec<Parameter>,
    /// Each lines in data section
    pub entities: Vec<EntityInstance>,
}

/// Each line of data section
#[derive(Debug, Clone, PartialEq)]
pub enum EntityInstance {
    Simple { name: u64, record: Record },
    Complex { name: u64, subsuper: Vec<Record> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceEntry {
    pub name: LValue,
    pub resource: URI,
}

#[derive(Debug, Clone, PartialEq)]
pub struct URI(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Anchor {
    pub name: String,
    pub item: AnchorItem,
    pub tags: Vec<(String, AnchorItem)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnchorItem {
    Integer(i64),
    Real(f64),
    String(String),
    Enumeration(String),
    /// The special token dollar sign (`$`) is used to represent an object whose value is not provided in the exchange structure.
    NotProvided,
    /// A reference to entity or value
    RValue(RValue),
    /// List of other parameters
    List(Vec<AnchorItem>),
}
