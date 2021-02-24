//! Parser for exchange structure
//!
//! Table 3 — WSN of the exchange structure
//! ----------------------------------------
//!
//! ```text
//! EXCHANGE_FILE = `ISO-10303-21;`
//!                 HEADER_SECTION
//!               \[ ANCHOR_SECTION \]
//!               \[ REFERENCE_SECTION \]
//!               { DATA_SECTION }
//!               `END-ISO-10303-21;`
//!               { SIGNATURE_SECTION } .
//!
//! SIGNATURE_SECTION  = `SIGNATURE` SIGNATURE_CONTENT `ENDSEC;`.
//! ```
//!

mod anchor;
mod data;
mod header;
mod parameter;
mod reference;

pub use anchor::*;
pub use data::*;
pub use header::*;
pub use parameter::*;
pub use reference::*;

use crate::parser::combinator::*;

pub struct Exchange {
    pub header: Vec<Record>,
    pub anchor: Vec<Anchor>,
    pub reference: Vec<ReferenceEntry>,
    pub data: Vec<DataSection>,
}

/// exchange_file = `ISO-10303-21;`
///                 [header_section]
///              \[ [anchor_section] \]
///              \[ [reference_section] \]
///               { [data_section] }
///                 `END-ISO-10303-21;`
///               { signature_section } .
pub fn exchange_file(input: &str) -> ParseResult<Exchange> {
    todo!()
}
