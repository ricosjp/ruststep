//! Reference of ruststep, a STEP toolkit written in pure Rust
//!
//! See [README.md][README] and [ARCHITECTURE.md][ARCHITECTURE] on [GitHub][GitHub]
//! for the overview and high-level architecture of this project.
//! This reference focus on the detail usage of this crate.
//!
//! [GitHub]: https://github.com/ricosjp/ruststep
//! [README]: https://github.com/ricosjp/ruststep/blob/master/README.md
//! [ARCHITECTURE]: https://github.com/ricosjp/ruststep/blob/master/ARCHITECTURE.md
//!

#![deny(rustdoc::broken_intra_doc_links)]

pub mod ast;
pub mod error;
pub mod header;
pub mod parser;
pub mod place_holder;
pub mod primitive;
pub mod tables;

// To work generated code by ruststep-derive only with ruststep
pub use derive_more;
pub use itertools;
pub use serde;

pub use ruststep_derive::*;

// Automatically generated codes
#[cfg(feature = "ap201")]
pub mod ap201;
#[cfg(feature = "ap203")]
pub mod ap203;
