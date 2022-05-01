use crate::{ast::*, error::*, parser::exchange::simple_record};
use std::str::FromStr;

/// A struct typed in EXPRESS schema, e.g. `A(1.0, 2.0)`
///
/// ```
/// use ruststep::ast::{Record, Parameter};
/// use std::str::FromStr;
///
/// let record = Record::from_str("A(1, 2)").unwrap();
/// assert_eq!(
///     record,
///     Record {
///         name: "A".to_string(),
///         parameter: Box::new(vec![Parameter::Integer(1), Parameter::Integer(2)].into())
///     }
/// )
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub name: String,
    pub parameter: Box<Parameter>,
}

impl FromStr for Record {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        use nom::Finish;
        let input = input.trim();
        let (_input, record) = simple_record(input)
            .finish()
            .map_err(|err| TokenizeFailed::new(input, err))?;
        Ok(record)
    }
}

impl AST for Record {}
