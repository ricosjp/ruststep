use super::{entity::*, expression::*, identifier::*, types::*, util::*};

#[derive(Debug, Clone, PartialEq)]
pub struct WhereClause {
    rules: Vec<DomainRule>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DomainRule {
    label: Option<String>,
    expr: Expression,
}

/// 338 where_clause = WHERE [domain_rule] `;` { [domain_rule] `;` } .
pub fn where_clause(input: &str) -> ParseResult<WhereClause> {
    tuple((tag("WHERE"), spaced_many0(tuple((domain_rule, char(';'))))))
        .map(|(_where, rules)| {
            let rules = rules.into_iter().map(|(rule, _semicoron)| rule).collect();
            WhereClause { rules }
        })
        .parse(input)
}

/// 202 domain_rule = \[ [rule_label_id] `:` \] [expression] .
pub fn domain_rule(input: &str) -> ParseResult<DomainRule> {
    tuple((opt(tuple((rule_label_id, char(':')))), expression))
        .map(|(opt, expr)| {
            let label = opt.map(|(label, _coron)| label);
            DomainRule { label, expr }
        })
        .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeriveClause {
    pub attributes: Vec<DerivedAttribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DerivedAttribute {
    pub attr: String,
    pub ty: ParameterType,
    pub expr: Expression,
}

/// 201 derive_clause = DERIVE [derived_attr] { [derived_attr] } .
pub fn derive_clause(input: &str) -> ParseResult<DeriveClause> {
    tuple((tag("DERIVE"), space_separated(derived_attr)))
        .map(|(_derive, attributes)| DeriveClause { attributes })
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

#[derive(Debug, Clone, PartialEq)]
pub struct InverseClause {
    pub attributes: Vec<InverseAttribute>,
}

/// 249 inverse_clause = INVERSE [inverse_attr] { [inverse_attr] } .
pub fn inverse_clause(input: &str) -> ParseResult<InverseClause> {
    tuple((tag("INVERSE"), space_separated(inverse_attr)))
        .map(|(_inverse, attributes)| InverseClause { attributes })
        .parse(input)
}

/// Attribute of an inverse clause parsed by [inverse_attr]
///
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

#[derive(Debug, Clone, PartialEq)]
pub struct UniqueClause {
    rules: Vec<UniqueRule>,
}

/// 333 unique_clause = UNIQUE [unique_rule] `;` { [unique_rule] `;` } .
pub fn unique_clause(input: &str) -> ParseResult<UniqueClause> {
    tuple((
        tag("UNIQUE"),
        space_separated(tuple((unique_rule, char(';')))),
    ))
    .map(|(_unique, seq)| UniqueClause {
        rules: seq.into_iter().map(|(rule, _semicoron)| rule).collect(),
    })
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

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn domain_rule() {
        let (residual, (rule, _remarks)) =
            super::domain_rule("notnegative : a > 0").finish().unwrap();
        assert_eq!(residual, "");
        assert_eq!(rule.label, Some("notnegative".to_string()));
    }

    #[test]
    fn where_clause() {
        let (residual, (w, _remarks)) = super::where_clause(
            r#"
            WHERE
                notnegative : SELF > 0;
            "#
            .trim(),
        )
        .finish()
        .unwrap();
        assert_eq!(residual, "");
        assert_eq!(w.rules.len(), 1);
        assert_eq!(w.rules[0].label, Some("notnegative".to_string()));
    }

    #[test]
    fn where_clause_complex() {
        let (residual, (w, _remarks)) = super::where_clause(
            r#"
            WHERE
                wr1: (1 <= SELF) AND (SELF <= 12);
            "#
            .trim(),
        )
        .finish()
        .unwrap();
        assert_eq!(residual, "");
        assert_eq!(w.rules.len(), 1);
        assert_eq!(w.rules[0].label, Some("wr1".to_string()));
    }
}
