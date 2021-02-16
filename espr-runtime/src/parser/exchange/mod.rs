//! Parser for exchange structure
//!
//! Table 3 — WSN of the exchange structure
//! ----------------------------------------
//!
//! ```text
//! EXCHANGE_FILE = "ISO-10303-21;" HEADER_SECTION
//!               [ ANCHOR_SECTION ]
//!               [ REFERENCE_SECTION ]
//!               { DATA_SECTION }
//!               "END-ISO-10303-21;"
//!               { SIGNATURE_SECTION } .
//!
//! SIGNATURE_SECTION  = "SIGNATURE" SIGNATURE_CONTENT "ENDSEC;".
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
