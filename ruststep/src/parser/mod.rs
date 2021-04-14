//! Tokenize ASCII exchange structure (or STEP-file) defined by ISO-10303-21
//!
//! This submodule responsible only for tokenize into abstract syntax tree (AST),
//! not for semantic validation.
//!
//! STEP-file consists of following sections:
//!
//! - HEADER
//! - ANCHOR (optional)
//! - REFERENCE (optional)
//! - DATA
//! - SIGNATURE (optional)
//!
//! ANCHOR, REFERENCE, and SIGNATURE sections are optional.
//! The syntax of STEP-file is common through schemas,
//! i.e. the tokenize of STEP-file can be done without reading any EXPRESS schema.
//! A target schema is specified in the HEADER section,
//! and it determines how we should understand AST.
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

use crate::{
    error::{Result, TokenizeFailed},
    step::*,
};
use nom::Finish;

/// Parse HEADER section
///
/// Example
/// --------
///
/// ```
/// let step_str = r#"
/// HEADER;
///   FILE_DESCRIPTION(('叛逆の物語', '魔法少女まどか☆マギカ'), '4;3');
///   FILE_NAME(
///     '/madoka/magica/rebellion.step',
///     '2013-10-26T10:30:00+09:00',
///     ('Mami Tomoe', 'Madoka Kaname', 'Sayaka Miki', 'Kyoko Sakura', 'Homura Akemi'),
///     ('Puella Magi Holy Quintet'),
///     'homu',
///     'Magica Quartet',
///     'qb@incubator.com'
///   );
///   FILE_SCHEMA(('MAGICAL_GIRL'));
/// ENDSEC;
/// "#.trim();
///
/// let (residual, header) = ruststep::parser::parse_header(&step_str).unwrap();
/// assert_eq!(residual, ""); // consume HEADER section of `step_str`
/// ```
pub fn parse_header(input: &str) -> Result<(&str, Vec<Record>)> {
    match exchange::header_section(input).finish() {
        Ok((input, records)) => Ok((input, records)),
        Err(e) => Err(TokenizeFailed::new(input, e).into()),
    }
}

/// Parse entire STEP file
pub fn parse(input: &str) -> Result<Exchange> {
    match exchange::exchange_file(input).finish() {
        Ok((_residual, ex)) => Ok(ex),
        Err(e) => Err(TokenizeFailed::new(input, e).into()),
    }
}
