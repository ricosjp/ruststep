//! Parse ASCII exchange structure defined by ISO-10303-21

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
