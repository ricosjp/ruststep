use super::super::{combinator::*, expression::*, identifier::*};
use crate::ast::algorithm::*;

/// 338 where_clause = WHERE [domain_rule] `;` { [domain_rule] `;` } .
pub fn where_clause(input: &str) -> ParseResult<WhereClause> {
    tuple((tag("WHERE"), many0(tuple((domain_rule, char(';'))))))
        .map(|(_where, rules)| {
            let rules = rules.into_iter().map(|(rule, _semicolon)| rule).collect();
            WhereClause { rules }
        })
        .parse(input)
}

/// 202 domain_rule = \[ [rule_label_id] `:` \] [expression] .
pub fn domain_rule(input: &str) -> ParseResult<DomainRule> {
    tuple((opt(tuple((rule_label_id, char(':')))), expression))
        .map(|(opt, expr)| {
            let label = opt.map(|(label, _colon)| label);
            DomainRule { label, expr }
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

    #[test]
    fn where_clause3() {
        let (residual, (w, _remarks)) = super::where_clause(
            r#"
            WHERE
                wr1 : VALUE_UNIQUE(s);
            "#
            .trim(),
        )
        .finish()
        .unwrap();
        assert_eq!(residual, "");
        dbg!(w);
    }
}
