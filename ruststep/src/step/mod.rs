//! Data structures in STEP file

mod logical;
mod parameter;
mod record;
mod value;

pub use logical::*;
pub use parameter::*;
pub use record::*;
pub use value::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Exchange {
    pub header: Vec<Record>,
    pub anchor: Vec<Anchor>,
    pub reference: Vec<ReferenceEntry>,
    pub data: Vec<DataSection>,
    pub signature: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataSection {
    pub meta: Vec<Parameter>,
    pub entities: Vec<EntityInstance>,
}

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
    /// A reference to entity or value, parsed by [rhs_occurrence_name]
    RValue(RValue),
    /// List of other parameters
    List(Vec<AnchorItem>),
}
