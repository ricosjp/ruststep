//! Syntatic analysis of EXPRESS language standardized as [ISO-10303-11](https://www.iso.org/standard/38047.html)
//!
//! This module is based on [nom](https://github.com/Geal/nom) parser combinater.

mod entity;
mod schema;
mod simple_data_type;

pub use entity::*;
pub use schema::*;
pub use simple_data_type::*;

use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*,
    number::complete::double, sequence::*, IResult, Parser,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Logical {
    False,
    True,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(u64),
    Real(f64),
    Logial(Logical),
}

/// 251 literal = binary_literal | logical_literal | real_literal | string_literal .
pub fn literal(input: &str) -> IResult<&str, Literal> {
    alt((
        binary_literal,
        logical_literal,
        real_literal,
        // FIXME string_literal
    ))
    .parse(input)
}

/// 139 binary_literal = `%` bit { bit } .
pub fn binary_literal(input: &str) -> IResult<&str, Literal> {
    todo!()
}

/// 255 logical_literal = `FALSE` | `TRUE` | `UNKNOWN` .
pub fn logical_literal(input: &str) -> IResult<&str, Literal> {
    alt((
        value(Literal::Logial(Logical::True), tag("TRUE")),
        value(Literal::Logial(Logical::False), tag("FALSE")),
        value(Literal::Logial(Logical::Unknown), tag("UNKNOWN")),
    ))
    .parse(input)
}

/// 141 integer_literal = digits .
///
/// Negative integer, e.g. `-23`,
/// will be represented by the combination of `-` unary operator and integer literal `23`
pub fn integer_literal(input: &str) -> IResult<&str, Literal> {
    digit1
        .map(|d: &str| Literal::Integer(d.parse().unwrap()))
        .parse(input)
}

/// 142 real_literal = integer_literal | ( digits `.` [ digits ] [ `e` [ sign ] digits ] ) .
pub fn real_literal(input: &str) -> IResult<&str, Literal> {
    double.map(|d| Literal::Real(d)).parse(input)
}

/// 143 simple_id = letter { letter | digit | ’_’ } .
///
/// Example
/// --------
///
/// ```
/// use exp2rs::parser;
/// use nom::Finish;
///
/// let (residual, id) = parser::simple_id("h").finish().unwrap();
/// assert_eq!(id, "h");
/// assert_eq!(residual, "");
///
/// let (residual, id) = parser::simple_id("homhom").finish().unwrap();
/// assert_eq!(id, "homhom");
/// assert_eq!(residual, "");
///
/// let (residual, id) = parser::simple_id("ho_mhom").finish().unwrap();
/// assert_eq!(id, "ho_mhom");
/// assert_eq!(residual, "");
///
/// let (residual, id) = parser::simple_id("h10o_1mh2om").finish().unwrap();
/// assert_eq!(id, "h10o_1mh2om");
/// assert_eq!(residual, "");
///
/// assert!(parser::simple_id("_homhom").finish().is_err());
/// assert!(parser::simple_id("1homhom").finish().is_err());
/// assert!(parser::simple_id("").finish().is_err());
/// ```
pub fn simple_id(input: &str) -> IResult<&str, String> {
    tuple((alpha1, many0(alt((alphanumeric1, is_a("_"))))))
        .map(|(head, tail)| format!("{}{}", head, tail.join("")))
        .parse(input)
}
