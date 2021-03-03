//! Parse ASCII exchange structure defined by ISO-10303-21
//!
//! ```
//! use std::{fs, path::*};
//!
//! // Read ABC Dataset's STEP file format example
//! let step_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//!     .join("tests/steps/00000050_80d90bfdd2e74e709956122a_step_000.step");
//! let step_str = fs::read_to_string(step_file).unwrap();
//!
//! // Parse STEP file into `Exchange` struct
//! let ex = ruststep::parser::parse(&step_str).unwrap();
//! ```

pub mod basic;
pub mod combinator;
pub mod exchange;
pub mod token;

use crate::error::*;
use nom::Finish;

pub fn parse(input: &str) -> Result<exchange::Exchange> {
    match exchange::exchange_file(input).finish() {
        Ok((_residual, ex)) => Ok(ex),
        Err(e) => {
            let error = nom::error::convert_error(input, e);
            Err(Error::ParseFailed(error))
        }
    }
}
