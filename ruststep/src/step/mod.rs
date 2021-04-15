//! Data structures in STEP file

mod logical;
mod parameter;
mod record;
mod value;

pub use logical::*;
pub use parameter::*;
pub use record::*;
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
