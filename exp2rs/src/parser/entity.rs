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

/// 9.2.1.1 Explicit attribute
///
/// ```text
/// 215 explicit_attr = attribute_decl { ’,’ attribute_decl } ’:’ [ OPTIONAL ] parameter_type ’;’ .
/// 177 attribute_decl = attribute_id | redeclared_attribute .
/// 266 parameter_type = generalized_types | named_types | simple_types .
/// ```
fn explicit_attr(input: &str) -> IResult<&str, (Vec<String>, String)> {
    tuple((
        separated_list0(tuple((space0, tag(","), space0)), identifier),
        space0,
        tag(":"),
        space0,
        identifier, // FIXME this must be type instead of identifier
        space0,
        tag(";"),
    ))
    .map(|(attrs, _, _, _, ty, _, _)| (attrs, ty))
    .parse(input)
}

fn entity_head(input: &str) -> IResult<&str, String> {
    tuple((tag("ENTITY"), space1, identifier, space0, tag(";")))
        .map(|(_, _, id, _, _)| id)
        .parse(input)
}

/// 9.2 Entity declaration
///
/// ```text
/// 206 entity_decl = entity_head entity_body END_ENTITY ’;’ .
/// 207 entity_head = ENTITY entity_id subsuper ’;’ .
/// 204 entity_body = { explicit_attr } [ derive_clause ] [ inverse_clause ] [ unique_clause ] [ where_clause ] .
/// ```
pub fn entity(input: &str) -> IResult<&str, Entity> {
    todo!()
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn entity_head() {
        let (residual, name) = super::entity_head("ENTITY homhom;").finish().unwrap();
        assert_eq!(name, "homhom");
        assert_eq!(residual, "");
    }

    #[test]
    fn explicit_attr() {
        let (residual, (id, ty)) = super::explicit_attr("x : Real;").finish().unwrap();
        assert_eq!(id, &["x"]);
        assert_eq!(ty, "Real");
        assert_eq!(residual, "");

        let (residual, (id, ty)) = super::explicit_attr("x, y : Real;").finish().unwrap();
        assert_eq!(id, &["x", "y"]);
        assert_eq!(ty, "Real");
        assert_eq!(residual, "");
    }
}
