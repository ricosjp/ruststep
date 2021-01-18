use super::{basis::*, expression::*, util::*};

#[derive(Debug, Clone, PartialEq)]
pub struct WhereClause {
    rules: Vec<DomainRule>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DomainRule {
    label: Option<String>,
    expr: Expression,
}

/// 338 where_clause = WHERE domain_rule `;` { domain_rule `;` } .
pub fn where_clause(input: &str) -> ParseResult<WhereClause> {
    tuple((tag("WHERE"), spaced_many0(tuple((domain_rule, char(';'))))))
        .map(|(_where, rules)| {
            let rules = rules.into_iter().map(|(rule, _semicoron)| rule).collect();
            WhereClause { rules }
        })
        .parse(input)
}

/// 202 domain_rule = \[ rule_label_id `:` \] expression .
///
/// `rule_label_id` is replaced by `simple_id` because
///
/// ```text
/// 294 rule_label_id = simple_id .
/// ```
pub fn domain_rule(input: &str) -> ParseResult<DomainRule> {
    tuple((opt(tuple((remarked(simple_id), char(':')))), expression))
        .map(|(opt, expr)| {
            let label = opt.map(|(label, _coron)| label);
            DomainRule { label, expr }
        })
        .parse(input)
}
