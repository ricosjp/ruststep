use super::attribute::*;
use crate::{
    ast::entity::*,
    parser::{combinator::*, identifier::*},
};

/// 333 unique_clause = UNIQUE [unique_rule] `;` { [unique_rule] `;` } .
pub fn unique_clause(input: &str) -> ParseResult<UniqueClause> {
    tuple((tag("UNIQUE"), many0(tuple((unique_rule, char(';'))))))
        .map(|(_unique, seq)| UniqueClause {
            rules: seq.into_iter().map(|(rule, _semicolon)| rule).collect(),
        })
        .parse(input)
}

/// 334 unique_rule = \[ [rule_label_id] `:` \] [referenced_attribute] { `,` [referenced_attribute] } .
pub fn unique_rule(input: &str) -> ParseResult<UniqueRule> {
    tuple((
        opt(tuple((rule_label_id, char(':')))),
        comma_separated(referenced_attribute),
    ))
    .map(|(opt_id, attributes)| UniqueRule {
        name: opt_id.map(|(name, _colon)| name),
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

    #[test]
    fn unique_clause_termination() {
        let (residual, (c, _remarks)) = super::unique_clause(
            r#"
            UNIQUE
              ur1 : revision_identifier1, drawing_identifier1;
              ur2 : revision_identifier2, drawing_identifier2;
            END_ENTITY;
            "#
            .trim(),
        )
        .finish()
        .unwrap();
        assert_eq!(residual, "END_ENTITY;");
        assert_eq!(c.rules.len(), 2);
    }
}
