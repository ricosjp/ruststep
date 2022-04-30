//! Abstract syntax tree for exchange structure
//!
//! This module contains implementation of [serde::Serialize] and [serde::Deserialize]
//! for AST structs.
//!
//! Mapping to serde data model
//! ----------------------------
//!
//! [Parameter] and [Record] can be deserialize through serde data model.
//!
//! | Parameter     | serde data model |
//! |:--------------|:-----------------|
//! | Integer       | i64              |
//! | Real          | f64              |
//! | String        | string           |
//! | Enumeration   | -                |
//! | List          | seq              |
//! | NotProvided   | unit             |
//! | Omitted       | unit             |
//! | Typed, Record | map              |
//! | RValue        | enum             |
//!
//! See [the official document of serde data model](https://serde.rs/data-model.html) for detail.
//!
//! - [Parameter::Typed] e.g. `A((1.0, 2.0))` and [Record] e.g. `A(1.0, 2.0)` are mapped to "map"
//!   in [the serde data model][serde-data-model]
//! - [Parameter::RValue] is mapped to "enum" in [the serde data model][serde-data-model].
//! - FIXME Enumeration is not supported yet.
//!
//! [serde-data-model]: https://serde.rs/data-model.html
//!

mod data_section;
mod parameter;
mod record;
mod ser;
mod value;

pub use data_section::*;
pub use parameter::*;
pub use record::*;
pub use ser::*;
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

/// Each line of data section
#[derive(Debug, Clone, PartialEq)]
pub enum EntityInstance {
    Simple { id: u64, record: Record },
    Complex { id: u64, subsuper: Vec<Record> },
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
