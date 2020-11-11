//! Syntatic analysis of EXPRESS language standardized as [ISO-10303-11](https://www.iso.org/standard/38047.html)
//!
//! This module is based on [nom](https://github.com/Geal/nom) parser combinater.

mod entity;
mod literal;
mod schema;
mod simple_data_type;

pub use entity::*;
pub use literal::*;
pub use schema::*;
pub use simple_data_type::*;

use nom::{
    branch::*, bytes::complete::*, character::complete::*, multi::*, sequence::*, IResult, Parser,
};

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
