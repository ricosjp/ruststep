use super::identifier;
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
pub struct Entity {
    /// Name of this entity type
    pub name: String,

    /// attribute name and types
    ///
    /// Be sure that this "type" is a string, not validated type in this timing
    pub attributes: Vec<(String, String)>,
}

fn entity_line(input: &str) -> IResult<&str, (String, String)> {
    todo!()
}

fn entity_header(input: &str) -> IResult<&str, String> {
    tuple((tag("ENTITY"), space1, identifier, space0, tag(";")))
        .map(|(_, _, id, _, _)| id)
        .parse(input)
}

pub fn entity(input: &str) -> IResult<&str, Entity> {
    todo!()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub name: String,
    pub entities: Vec<Entity>,
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn entity_header() {
        let (residual, name) = super::entity_header("ENTITY homhom;").finish().unwrap();
        assert_eq!(name, "homhom");
        assert_eq!(residual, "");
    }
}
