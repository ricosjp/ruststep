use super::{basis::*, simple_data_type::*, util::*};
use derive_more::From;
use nom::{branch::*, bytes::complete::*, character::complete::*, sequence::*, IResult, Parser};

/// Parsed result of EXPRESS's ENTITY
#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    /// Name of this entity type
    pub name: String,

    /// attribute name and types
    ///
    /// Be sure that this "type" is a string, not validated type in this timing
    pub attributes: Vec<(String, ParameterType)>,
}

#[derive(Debug, Clone, PartialEq, From)]
pub enum ParameterType {
    Named(String),
    Simple(SimpleType),
}

/// 266 parameter_type = generalized_types | named_types | simple_types .
pub fn paramter_type(input: &str) -> IResult<&str, ParameterType> {
    // FIXME generalized_types
    // FIXME named_types
    alt((
        simple_id.map(|ty| ParameterType::Named(ty)),
        simple_types.map(|ty| ParameterType::Simple(ty)),
    ))
    .parse(input)
}

/// 215 explicit_attr = attribute_decl { `,` attribute_decl } `:` \[ OPTIONAL \] parameter_type `;` .
pub fn explicit_attr(input: &str) -> IResult<&str, (Vec<String>, ParameterType)> {
    // FIXME Support attribute_decl
    // FIXME OPTIONAL
    tuple((
        comma_separated(simple_id),
        multispace0,
        tag(":"),
        multispace0,
        paramter_type,
        multispace0,
        tag(";"),
    ))
    .map(|(attrs, _, _, _, ty, _, _)| (attrs, ty))
    .parse(input)
}

/// 207 entity_head = ENTITY entity_id subsuper `;` .
pub fn entity_head(input: &str) -> IResult<&str, String> {
    tuple((tag("ENTITY"), multispace1, simple_id, multispace0, tag(";")))
        .map(|(_, _, id, _, _)| id)
        .parse(input)
}

/// 206 entity_decl = entity_head entity_body END_ENTITY `;` .
pub fn entity_decl(input: &str) -> IResult<&str, Entity> {
    tuple((
        entity_head,
        multispace0,
        spaced_many0(explicit_attr),
        multispace0,
        tag("END_ENTITY"),
        multispace0,
        tag(";"),
    ))
    .map(|(name, _, attributes, _, _, _, _)| Entity {
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
    use super::*;
    use nom::Finish;

    #[test]
    fn entity_head() {
        let (residual, name) = super::entity_head("ENTITY homhom;").finish().unwrap();
        assert_eq!(name, "homhom");
        assert_eq!(residual, "");
    }

    #[test]
    fn explicit_attr() {
        let (residual, (id, ty)) = super::explicit_attr("x : REAL;").finish().unwrap();
        assert_eq!(id, &["x"]);
        assert!(matches!(ty, ParameterType::Simple(SimpleType::Real)));
        assert_eq!(residual, "");

        let (residual, (id, ty)) = super::explicit_attr("x, y : REAL;").finish().unwrap();
        assert_eq!(id, &["x", "y"]);
        assert!(matches!(ty, ParameterType::Simple(SimpleType::Real)));
        assert_eq!(residual, "");
    }

    #[test]
    fn entity_decl() {
        let exp_str = r#"
        ENTITY first;
          m_ref : second;
          fattr : REAL;
        END_ENTITY;
        "#
        .trim();

        let (residual, entity) = super::entity_decl(exp_str).finish().unwrap();
        assert_eq!(entity.name, "first");

        assert_eq!(entity.attributes.len(), 2);
        // check `m_ref`
        assert_eq!(entity.attributes[0].0, "m_ref");
        assert!(matches!(entity.attributes[0].1, ParameterType::Named(_)));
        // check `fattr`
        assert_eq!(entity.attributes[1].0, "fattr");
        assert!(matches!(
            entity.attributes[1].1,
            ParameterType::Simple(SimpleType::Real)
        ));

        assert_eq!(residual, "");
    }
}
