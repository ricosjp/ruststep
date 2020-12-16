//! About EXPRESS Language
//! -----------------------
//!
//! EXPRESS language standardized as [ISO-10303-11](https://www.iso.org/standard/38047.html).
//!

pub mod decode;
pub mod parser;
pub mod semantics;
pub mod types;

pub use decode::decode;
pub use iso_10303_simple_types::*;
