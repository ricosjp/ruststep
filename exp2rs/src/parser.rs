//! Syntatic analysis of EXPRESS language
//!
//! EXPRESS language is standardized as [ISO-10303-11][ISO-10303-11].
//!
//! [ISO-10303-11]: https://www.iso.org/standard/38047.html

use nom::{
    branch::*, bytes::complete::*, character::complete::*, multi::*, sequence::*, IResult, Parser,
};

/// ```text
/// ENTITY first;
/// m_ref : second;
/// fattr : STRING;
/// END_ENTITY;
/// ```
#[derive(Debug, Clone, PartialEq)]
struct Entity {
    /// Name of this entity type
    name: String,

    /// attribute name and types
    ///
    /// Be sure that this "type" is a string, not validated type in this timing
    attributes: Vec<(String, String)>,
}

fn entity_line(input: &str) -> IResult<&str, (String, String)> {
    todo!()
}

/// Parse identifier
///
/// ```text
/// 143 simple_id = letter { letter | digit | ’_’ } .
/// 128 letter = ’a’ | ’b’ | ’c’ | ’d’ | ’e’ | ’f’ | ’g’ | ’h’ | ’i’ | ’j’ | ’k’ | ’l’ |’m’ | ’n’ | ’o’ | ’p’ | ’q’ | ’r’ | ’s’ | ’t’ | ’u’ | ’v’ | ’w’ | ’x’ |’y’ | ’z’ .
/// 124 digit = ’0’ | ’1’ | ’2’ | ’3’ | ’4’ | ’5’ | ’6’ | ’7’ | ’8’ | ’9’
/// ```
fn identifier(input: &str) -> IResult<&str, String> {
    tuple((alpha1, many0(alt((alphanumeric1, is_a("_"))))))
        .map(|(head, tail)| format!("{}{}", head, tail.join("")))
        .parse(input)
}

fn entity_header(input: &str) -> IResult<&str, String> {
    tuple((tag("ENTITY"), space1, identifier, space0, tag(";")))
        .map(|(_, _, id, _, _)| id)
        .parse(input)
}

fn entity(input: &str) -> IResult<&str, Entity> {
    todo!()
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

    #[test]
    fn entity_header() {
        let (residual, name) = super::entity_header("ENTITY homhom;").finish().unwrap();
        assert_eq!(name, "homhom");
        assert_eq!(residual, "");
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Schema {
    name: String,
    entities: Vec<Entity>,
}
