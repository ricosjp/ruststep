//! Tokenize ASCII exchange structure defined by ISO-10303-21
//!
//! Be sure that this submodule responsible only for tokenize into abstract syntax tree (AST),
//! not for semantic validation.
//!
//! Example
//! --------
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

use nom::Finish;
use std::fmt;

/// Error while tokenizing STEP input
#[derive(Debug)]
pub struct TokenizeFailed {
    rendered_error: String,
}

impl fmt::Display for TokenizeFailed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "Error while tokenizing STEP input\n{}",
            self.rendered_error
        )?;
        Ok(())
    }
}

impl std::error::Error for TokenizeFailed {}

impl TokenizeFailed {
    fn new(input: &str, err: nom::error::VerboseError<&str>) -> Self {
        TokenizeFailed {
            rendered_error: nom::error::convert_error(input, err),
        }
    }
}

pub fn parse(input: &str) -> Result<exchange::Exchange, TokenizeFailed> {
    match exchange::exchange_file(input).finish() {
        Ok((_residual, ex)) => Ok(ex),
        Err(e) => Err(TokenizeFailed::new(input, e)),
    }
}
