use super::super::{expression::*, identifier::*, util::*};

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
        comma_separated(referenced_attribute),
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
    fn unique_clause() {
        let (residual, (c, _remarks)) = super::unique_clause(
            r#"
            UNIQUE
              ur1 : revision_identifier, drawing_identifier;
            "#
            .trim(),
        )
        .finish()
        .unwrap();
        assert_eq!(residual, "");
        assert_eq!(c.rules.len(), 1);
    }
}
