use super::{expression::*, identifier::*, types::*, util::*};
use derive_more::From;

/// Parsed result of EXPRESS's ENTITY
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    /// Name of this entity type
    pub name: String,

    /// attribute name and types
    ///
    /// Be sure that this "type" is a string, not validated type in this timing
    pub attributes: Vec<EntityAttribute>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityAttribute {
    pub name: String,
    pub ty: ParameterType,
    pub optional: bool,
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

/// 266 parameter_type = generalized_types | [named_types] | [simple_types] .
pub fn parameter_type(input: &str) -> ParseResult<ParameterType> {
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

/// 215 explicit_attr = [attribute_decl] { `,` [attribute_decl] } `:` \[ OPTIONAL \] [parameter_type] `;` .
pub fn explicit_attr(input: &str) -> ParseResult<Vec<EntityAttribute>> {
    tuple((
        comma_separated(attribute_decl),
        char(':'),
        opt(tag("OPTIONAL")),
        parameter_type,
        char(';'),
    ))
    .map(|(attrs, _coron, optional, ty, _semicoron)| {
        attrs
            .into_iter()
            .map(|name| EntityAttribute {
                name,
                ty: ty.clone(),
                optional: optional.is_some(),
            })
            .collect()
    })
    .parse(input)
}

/// 207 entity_head = ENTITY [entity_id] subsuper `;` .
pub fn entity_head(input: &str) -> ParseResult<String> {
    // FIXME subsuper
    tuple((
        tag("ENTITY "), // parse with trailing space
        entity_id,
        char(';'),
    ))
    .map(|(_start, id, _semicoron)| id)
    .parse(input)
}

/// 204 entity_body = { [explicit_attr] } \[ derive_clause \] \[ inverse_clause \] \[ unique_clause \] \[ where_clause \] .
pub fn entity_body(input: &str) -> ParseResult<Vec<EntityAttribute>> {
    // FIXME derive_clause
    // FIXME inverse_clause
    // FIXME unique_clause
    // FIXME where_clause
    spaced_many0(explicit_attr)
        .map(|attributes| attributes.into_iter().flatten().collect())
        .parse(input)
}

/// 206 entity_decl = [entity_head] [entity_body] END_ENTITY `;` .
pub fn entity_decl(input: &str) -> ParseResult<Entity> {
    tuple((entity_head, entity_body, tag("END_ENTITY"), char(';')))
        .map(|(name, attributes, _end, _semicoron)| Entity { name, attributes })
        .parse(input)
}

/// Constructor like `point(0.0, 0.0, 0.0)`
#[derive(Debug, Clone, PartialEq)]
pub struct EntityConstructor {
    name: String,
    values: Vec<Expression>,
}

/// 205 entity_constructor = [entity_ref] ’(’ \[ [expression] { ’,’ [expression] } \] ’)’ .
pub fn entity_constructor(input: &str) -> ParseResult<EntityConstructor> {
    tuple((
        entity_ref,
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
        let (residual, (attrs, _remark)) = super::explicit_attr("x : REAL;").finish().unwrap();
        assert_eq!(residual, "");
        assert_eq!(attrs.len(), 1);
        let attr = &attrs[0];
        assert_eq!(attr.name, "x");
        assert!(matches!(attr.ty, ParameterType::Simple(SimpleType::Real)));

        let (residual, (attrs, _remark)) = super::explicit_attr("x, y : REAL;").finish().unwrap();
        assert_eq!(residual, "");
        assert_eq!(attrs.len(), 2);
        let attr = &attrs[0];
        assert_eq!(attr.name, "x");
        assert!(matches!(attr.ty, ParameterType::Simple(SimpleType::Real)));
        let attr = &attrs[1];
        assert_eq!(attr.name, "y");
        assert!(matches!(attr.ty, ParameterType::Simple(SimpleType::Real)));
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
        assert_eq!(entity.attributes[0].name, "m_ref");
        assert!(matches!(entity.attributes[0].ty, ParameterType::Named(_)));
        // check `fattr`
        assert_eq!(entity.attributes[1].name, "fattr");
        assert!(matches!(
            entity.attributes[1].ty,
            ParameterType::Simple(SimpleType::Real)
        ));

        assert_eq!(residual, "");
    }
}
