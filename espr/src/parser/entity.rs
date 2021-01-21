use super::{basis::*, expression::*, identifier::*, types::*, util::*};
use derive_more::From;

/// Parsed result of EXPRESS's ENTITY
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    /// Name of this entity type
    pub name: String,

    /// attribute name and types
    ///
    /// Be sure that this "type" is a string, not validated type in this timing
    pub attributes: Vec<(String, ParameterType)>,
}

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum ParameterType {
    Named(String),
    Simple(SimpleType),
}

/// 258 named_types = [entity_ref] | [type_ref] .
pub fn named_types(input: &str) -> ParseResult<String> {
    alt((entity_ref, type_ref)).parse(input)
}

/// 266 parameter_type = generalized_types | named_types | simple_types .
pub fn paramter_type(input: &str) -> ParseResult<ParameterType> {
    // FIXME generalized_types
    alt((
        named_types.map(|ty| ParameterType::Named(ty)),
        simple_types.map(|ty| ParameterType::Simple(ty)),
    ))
    .parse(input)
}

/// 177 attribute_decl = [attribute_id] | redeclared_attribute .
pub fn attribute_decl(input: &str) -> ParseResult<String> {
    // FIXME Support redeclared_attribute
    attribute_id(input)
}

/// 215 explicit_attr = attribute_decl { `,` attribute_decl } `:` \[ OPTIONAL \] parameter_type `;` .
pub fn explicit_attr(input: &str) -> ParseResult<(Vec<String>, ParameterType)> {
    // FIXME OPTIONAL
    tuple((
        comma_separated(attribute_decl),
        char(':'),
        paramter_type,
        char(';'),
    ))
    .map(|(attrs, _coron, ty, _semicoron)| (attrs, ty))
    .parse(input)
}

/// 207 entity_head = ENTITY entity_id subsuper `;` .
pub fn entity_head(input: &str) -> ParseResult<String> {
    tuple((
        tag("ENTITY "), // parse with trailing space
        entity_id,
        char(';'),
    ))
    .map(|(_start, id, _semicoron)| id)
    .parse(input)
}

/// 206 entity_decl = entity_head entity_body END_ENTITY `;` .
pub fn entity_decl(input: &str) -> ParseResult<Entity> {
    tuple((
        entity_head,
        spaced_many0(explicit_attr),
        tag("END_ENTITY"),
        char(';'),
    ))
    .map(|(name, attributes, _end, _semicoron)| {
        let attributes = attributes
            .into_iter()
            .map(|(attrs, ty)| attrs.into_iter().map(move |attr| (attr, ty.clone())))
            .flatten()
            .collect();
        Entity { name, attributes }
    })
    .parse(input)
}

/// Constructor like `point(0.0, 0.0, 0.0)`
#[derive(Debug, Clone, PartialEq)]
pub struct EntityConstructor {
    name: String,
    values: Vec<Expression>,
}

/// 205 entity_constructor = entity_ref ’(’ [ expression { ’,’ expression } ] ’)’ .
pub fn entity_constructor(input: &str) -> ParseResult<EntityConstructor> {
    tuple((
        remarked(simple_id),
        char('('),
        opt(comma_separated(expression)),
        char(')'),
    ))
    .map(|(name, _open, values, _close)| EntityConstructor {
        name,
        values: values.unwrap_or(Vec::new()),
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Finish;

    #[test]
    fn entity_head() {
        let (residual, (name, _remark)) = super::entity_head("ENTITY homhom;").finish().unwrap();
        assert_eq!(name, "homhom");
        assert_eq!(residual, "");
    }

    #[test]
    fn explicit_attr() {
        let (residual, ((id, ty), _remark)) = super::explicit_attr("x : REAL;").finish().unwrap();
        assert_eq!(id, &["x"]);
        assert!(matches!(ty, ParameterType::Simple(SimpleType::Real)));
        assert_eq!(residual, "");

        let (residual, ((id, ty), _remark)) =
            super::explicit_attr("x, y : REAL;").finish().unwrap();
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

        let (residual, (entity, _remark)) = super::entity_decl(exp_str).finish().unwrap();
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

    #[test]
    fn entity_constructor() {
        let (residual, (ctor, _remarks)) = super::entity_constructor("point(0.0, 0.0, 0.0)")
            .finish()
            .unwrap();
        assert_eq!(residual, "");
        assert_eq!(ctor.name, "point");
        assert_eq!(ctor.values.len(), 3);
    }
}
