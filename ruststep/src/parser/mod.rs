//! Parse ASCII exchange structure defined by ISO-10303-21

pub mod basic;
pub mod combinator;
pub mod exchange;
pub mod token;

use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    errors: Vec<String>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for error in &self.errors {
            write!(f, "{}", error)?;
        }
        Ok(())
    }
}

impl std::error::Error for ParseError {}

pub fn parse(input: &str) -> Result<exchange::Exchange, ParseError> {
    todo!()
}
