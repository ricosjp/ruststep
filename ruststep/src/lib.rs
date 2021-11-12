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

pub use derive_more;
pub use ruststep_derive::*;

// Automatically generated codes
#[cfg(feature = "ap201")]
pub mod ap201;

#[cfg(all(feature = "ap201", test))]
mod ap201_test {
    use super::ap201::explicit_draughting::*;
    use super::tables::*;
    use std::str::FromStr;

    fn example() -> Tables {
        Tables::from_str(
            r#"
            DATA;
              #1 = COLOUR_RGB(1.0, 0.0, 0.0);
            ENDSEC;
            "#,
        )
        .unwrap()
    }

    #[test]
    fn get_owned() {
        let tables = example();
        let colour = EntityTable::<ColourRgbHolder>::get_owned(&tables, 1).unwrap();
        dbg!(colour);
    }
}
