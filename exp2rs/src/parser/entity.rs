use super::identifier;
use nom::{bytes::complete::*, character::complete::*, multi::*, sequence::*, IResult, Parser};

/// Parsed result of EXPRESS's ENTITY
///
/// Example
/// --------
///
/// ```
/// use exp2rs::parser;
/// use nom::Finish;
///
/// let exp_str = r#"
/// ENTITY first;
///   m_ref : second;
///   fattr : STRING;
/// END_ENTITY;
/// "#.trim();
///
/// let (residual, entity) = parser::entity(exp_str).finish().unwrap();
/// assert_eq!(entity.name, "first");
/// assert_eq!(
///     entity.attributes,
///     &[
///         ("m_ref".to_string(), "second".to_string()),
///         ("fattr".to_string(), "STRING".to_string())
///     ]
/// );
/// assert_eq!(residual, "");
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
pub fn explicit_attr(input: &str) -> IResult<&str, (Vec<String>, String)> {
    tuple((
        separated_list1(tuple((multispace0, tag(","), multispace0)), identifier),
        multispace0,
        tag(":"),
        multispace0,
        identifier, // FIXME this must be type instead of identifier
        multispace0,
        tag(";"),
    ))
    .map(|(attrs, _, _, _, ty, _, _)| (attrs, ty))
    .parse(input)
}

fn entity_head(input: &str) -> IResult<&str, String> {
    tuple((
        tag("ENTITY"),
        multispace1,
        identifier,
        multispace0,
        tag(";"),
    ))
    .map(|(_, _, id, _, _)| id)
    .parse(input)
}

fn entity_end(input: &str) -> IResult<&str, ()> {
    tuple((tag("END_ENTITY"), multispace0, tag(";")))
        .map(|_| ())
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
    tuple((
        entity_head,
        multispace0,
        separated_list0(multispace0, explicit_attr),
        multispace0,
        entity_end,
    ))
    .map(|(name, _, attributes, _, _)| Entity {
        name,
        attributes: attributes
            .into_iter()
            .map(|(attrs, ty)| attrs.into_iter().map(move |attr| (attr, ty.clone())))
            .flatten()
            .collect(),
    })
    .parse(input)
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
