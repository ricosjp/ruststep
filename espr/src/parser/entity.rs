use super::{domain::*, expression::*, identifier::*, subsuper::*, types::*, util::*};

/// Parsed result of EXPRESS's ENTITY
#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    /// Name of this entity type
    pub name: String,

    /// attribute name and types
    ///
    /// Be sure that this "type" is a string, not validated type in this timing
    pub attributes: Vec<EntityAttribute>,

    pub constraint: Option<Constraint>,
    pub subtype: Option<SubTypeDecl>,
    pub where_clause: Option<WhereClause>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityAttribute {
    pub name: String,
    pub ty: ParameterType,
    pub optional: bool,
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

/// 207 entity_head = ENTITY [entity_id] [subsuper] `;` .
pub fn entity_head(input: &str) -> ParseResult<(String, Option<Constraint>, Option<SubTypeDecl>)> {
    tuple((
        tag("ENTITY "), // parse with trailing space
        entity_id,
        subsuper,
        char(';'),
    ))
    .map(|(_start, id, (constraint, subtype), _semicoron)| (id, constraint, subtype))
    .parse(input)
}

/// 204 entity_body = { [explicit_attr] } \[ derive_clause \] \[ inverse_clause \] \[ unique_clause \] \[ [where_clause] \] .
pub fn entity_body(input: &str) -> ParseResult<(Vec<EntityAttribute>, Option<WhereClause>)> {
    // FIXME derive_clause
    // FIXME inverse_clause
    // FIXME unique_clause
    tuple((spaced_many0(explicit_attr), opt(where_clause)))
        .map(|(attributes, where_clause)| {
            (attributes.into_iter().flatten().collect(), where_clause)
        })
        .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub struct DerivedAttribute {
    pub attr: String,
    pub ty: ParameterType,
    pub expr: Expression,
}

/// 201 derive_clause = DERIVE [derived_attr] { [derived_attr] } .
pub fn derive_clause(input: &str) -> ParseResult<Vec<DerivedAttribute>> {
    tuple((tag("DERIVE"), space_separated(derived_attr)))
        .map(|(_derive, attrs)| attrs)
        .parse(input)
}

/// 200 derived_attr = [attribute_decl] `:` [parameter_type] `:=` [expression] `;` .
pub fn derived_attr(input: &str) -> ParseResult<DerivedAttribute> {
    tuple((
        attribute_decl,
        char(':'),
        parameter_type,
        tag(":="),
        expression,
        char(';'),
    ))
    .map(|(attr, _coron, ty, _equal, expr, _semicoron)| DerivedAttribute { attr, ty, expr })
    .parse(input)
}

/// 249 inverse_clause = INVERSE [inverse_attr] { [inverse_attr] } .
pub fn inverse_clause(input: &str) -> ParseResult<Vec<InverseAttribute>> {
    tuple((tag("INVERSE"), space_separated(inverse_attr)))
        .map(|(_inverse, attrs)| attrs)
        .parse(input)
}

/// From ISO 10303-11 document,
///
/// ```text
/// ENTITY door;
///   handle : knob;                -- inverse relationship for this attribute
///   hinges : SET [1:?] OF hinge;
/// END_ENTITY;
///
/// ENTITY knob;
/// ...
/// INVERSE
///   opens : door FOR handle;
/// (* ^      ^        ^
///    |      |        attribute name used in the parent entity
///    |      The entity which has `SELF` as attribute
///    name of this inverse relationship *)
/// END_ENTITY;
/// ```
///
/// This means
///
/// > knobs can only exist if they are used in the role of handle in one instance of a door
///
#[derive(Debug, Clone, PartialEq)]
pub struct InverseAttribute {
    /// Name of this inverse relationship
    ///
    /// `opens` in above example
    name: String,

    /// The entity name which has `SELF` as an attribute
    ///
    /// `door` in above example
    dest: String,

    /// Used if `SET` or `BAG` for parent entity specification
    dest_aggregation: AggregationOption,

    /// The attribute name used in the parent entity
    ///
    /// `handle` in above example
    attribute: String,

    /// Prefix of the attribute, used if the attribute is a sub-attribute of `dest` entity
    attribute_prefix: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AggregationOption {
    Set { bound: Option<Bound> },
    Bag { bound: Option<Bound> },
    None,
}

/// 248 inverse_attr = [attribute_decl] `:`
///                  \[ ( SET | BAG ) \[ [bound_spec] \] OF \]
///                  [entity_ref] FOR
///                  \[ [entity_ref] `.` \]
///                  [attribute_ref] `;` .
pub fn inverse_attr(input: &str) -> ParseResult<InverseAttribute> {
    let aggregation_option = opt(tuple((
        alt((tag("SET"), tag("BAG"))),
        opt(bound_spec),
        tag("OF"),
    )))
    .map(|opt| {
        if let Some((agg, bound, _of)) = opt {
            match agg {
                "SET" => AggregationOption::Set { bound },
                "BAG" => AggregationOption::Bag { bound },
                _ => unreachable!(),
            }
        } else {
            AggregationOption::None
        }
    });

    let attribute_prefix =
        opt(tuple((entity_ref, char('.')))).map(|opt| opt.map(|(prefix, _dot)| prefix));

    tuple((
        attribute_decl,
        char(':'),
        aggregation_option,
        entity_ref,
        tag("FOR"),
        attribute_prefix,
        attribute_ref,
        char(';'),
    ))
    .map(
        |(name, _comma, dest_aggregation, dest, _for, attribute_prefix, attribute, _semicoron)| {
            InverseAttribute {
                name,
                dest,
                dest_aggregation,
                attribute,
                attribute_prefix,
            }
        },
    )
    .parse(input)
}

/// 333 unique_clause = UNIQUE [unique_rule] `;` { [unique_rule] `;` } .
pub fn unique_clause(input: &str) -> ParseResult<Vec<UniqueRule>> {
    tuple((
        tag("UNIQUE"),
        space_separated(tuple((unique_rule, char(';')))),
    ))
    .map(|(_unique, seq)| seq.into_iter().map(|(rule, _semicoron)| rule).collect())
    .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub struct UniqueRule {
    pub name: Option<String>,
    pub attributes: Vec<ReferencedAttribute>,
}

/// 334 unique_rule = \[ [rule_label_id] `:` \] [referenced_attribute] { `,` [referenced_attribute] } .
pub fn unique_rule(input: &str) -> ParseResult<UniqueRule> {
    tuple((
        opt(tuple((rule_label_id, char(':')))),
        space_separated(referenced_attribute),
    ))
    .map(|(opt_id, attributes)| UniqueRule {
        name: opt_id.map(|(name, _coron)| name),
        attributes,
    })
    .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReferencedAttribute {
    Reference(String),
    QualifiedAttribute(Expression),
}

/// 280 referenced_attribute = [attribute_ref] | [qualified_attribute] .
pub fn referenced_attribute(input: &str) -> ParseResult<ReferencedAttribute> {
    alt((
        attribute_ref.map(|r| ReferencedAttribute::Reference(r)),
        qualified_attribute.map(|a| ReferencedAttribute::QualifiedAttribute(a)),
    ))
    .parse(input)
}

/// 275 qualified_attribute = SELF [group_qualifier] [attribute_qualifier] .
pub fn qualified_attribute(input: &str) -> ParseResult<Expression> {
    tuple((tag("SELF"), opt(group_qualifier), opt(attribute_qualifier)))
        .map(|(_self, group, attr)| {
            let mut qualifiers = Vec::new();
            if let Some(group) = group {
                qualifiers.push(group)
            }
            if let Some(attr) = attr {
                qualifiers.push(attr)
            }
            Expression::self_qualified(qualifiers)
        })
        .parse(input)
}

/// 206 entity_decl = [entity_head] [entity_body] END_ENTITY `;` .
pub fn entity_decl(input: &str) -> ParseResult<Entity> {
    tuple((entity_head, entity_body, tag("END_ENTITY"), char(';')))
        .map(
            |((name, constraint, subtype), (attributes, where_clause), _end, _semicoron)| Entity {
                name,
                attributes,
                constraint,
                subtype,
                where_clause,
            },
        )
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
        let (residual, ((name, constraint, subtype), _remark)) =
            super::entity_head("ENTITY homhom;").finish().unwrap();
        assert_eq!(name, "homhom");
        assert_eq!(constraint, None);
        assert_eq!(subtype, None);
        assert_eq!(residual, "");
    }

    #[test]
    fn subtype_of() {
        // example from ISO 10303-11:2004(E) p.50
        let (residual, ((name, constraint, subtype), _remark)) =
            super::entity_head("ENTITY odd_number SUBTYPE OF (integer_number);")
                .finish()
                .unwrap();
        assert_eq!(name, "odd_number");
        assert_eq!(constraint, None);
        assert_eq!(
            subtype,
            Some(SubTypeDecl {
                entity_references: vec!["integer_number".to_string()]
            })
        );
        assert_eq!(residual, "");
    }

    #[test]
    fn abstract_entity() {
        // example from ISO 10303-11:2004(E) p.52
        let (residual, ((name, constraint, subtype), _remark)) =
            super::entity_head("ENTITY line ABSTRACT;")
                .finish()
                .unwrap();
        assert_eq!(name, "line");
        assert_eq!(constraint, Some(Constraint::AbstractEntity));
        assert_eq!(subtype, None);
        assert_eq!(residual, "");
    }

    #[test]
    fn one_of() {
        // example from ISO 10303-11:2004(E) p.57 with some modification
        let (residual, ((name, constraint, subtype), _remark)) =
            super::entity_head("ENTITY pet ABSTRACT SUPERTYPE OF (ONEOF(cat, rabbit, dog));")
                .finish()
                .unwrap();
        assert_eq!(name, "pet");
        assert_eq!(
            constraint,
            Some(Constraint::AbstractSuperType(Some(
                SuperTypeExpression::OneOf {
                    exprs: vec![
                        SuperTypeExpression::Reference("cat".to_string()),
                        SuperTypeExpression::Reference("rabbit".to_string()),
                        SuperTypeExpression::Reference("dog".to_string())
                    ]
                }
            )))
        );
        assert_eq!(subtype, None);
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
