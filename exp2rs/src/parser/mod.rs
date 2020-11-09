//! Syntatic analysis of EXPRESS language standardized as [ISO-10303-11](https://www.iso.org/standard/38047.html)
//!
//! This module is based on [nom](https://github.com/Geal/nom) parser combinater.

mod entity;
mod schema;

pub use entity::*;
pub use schema::*;

use nom::{
    branch::*, bytes::complete::*, character::complete::*, multi::*, sequence::*, IResult, Parser,
};

/// 7.4 Identifier
///
/// ```text
/// 143 simple_id = letter { letter | digit | ’_’ } .
/// 128 letter = ’a’ | ’b’ | ’c’ | ’d’ | ’e’ | ’f’ | ’g’ | ’h’ | ’i’ | ’j’ | ’k’ | ’l’ |’m’ | ’n’ | ’o’ | ’p’ | ’q’ | ’r’ | ’s’ | ’t’ | ’u’ | ’v’ | ’w’ | ’x’ |’y’ | ’z’ .
/// 124 digit = ’0’ | ’1’ | ’2’ | ’3’ | ’4’ | ’5’ | ’6’ | ’7’ | ’8’ | ’9’ .
/// ```
pub fn identifier(input: &str) -> IResult<&str, String> {
    tuple((alpha1, many0(alt((alphanumeric1, is_a("_"))))))
        .map(|(head, tail)| format!("{}{}", head, tail.join("")))
        .parse(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn identifier() {
        let (residual, id) = super::identifier("h").finish().unwrap();
        assert_eq!(id, "h");
        assert_eq!(residual, "");

        let (residual, id) = super::identifier("homhom").finish().unwrap();
        assert_eq!(id, "homhom");
        assert_eq!(residual, "");

        let (residual, id) = super::identifier("ho_mhom").finish().unwrap();
        assert_eq!(id, "ho_mhom");
        assert_eq!(residual, "");

        let (residual, id) = super::identifier("h10o_1mh2om").finish().unwrap();
        assert_eq!(id, "h10o_1mh2om");
        assert_eq!(residual, "");

        assert!(super::identifier("_homhom").finish().is_err());
        assert!(super::identifier("1homhom").finish().is_err());
        assert!(super::identifier("").finish().is_err());
    }
}
