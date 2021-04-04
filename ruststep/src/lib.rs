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

#![deny(broken_intra_doc_links)]

pub mod error;
pub mod header;
pub mod parser;
pub mod step;
pub mod tables;

pub mod ap000;
