use super::super::{attribute::*, identifier::*, util::*};

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
    pub attributes: Vec<AttributeDecl>,
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
