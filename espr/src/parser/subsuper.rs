use super::{combinator::*, identifier::*};
use crate::ast::*;

/// 164 abstract_entity_declaration = ABSTRACT .
pub fn abstract_entity_declaration(input: &str) -> ParseResult<Constraint> {
    tag("ABSTRACT")
        .map(|_| Constraint::AbstractEntity)
        .parse(input)
}

/// 166 abstract_supertype_declaration = ABSTRACT SUPERTYPE \[ [subtype_constraint] \] .
pub fn abstract_supertype_declaration(input: &str) -> ParseResult<Constraint> {
    tuple((tag("ABSTRACT"), tag("SUPERTYPE"), opt(subtype_constraint)))
        .map(|(_abstract, _supertype, expr)| Constraint::AbstractSuperType(expr))
        .parse(input)
}

/// 312 subsuper = \[ [supertype_constraint] \] \[ [subtype_declaration] \] .
pub fn subsuper(input: &str) -> ParseResult<(Option<Constraint>, Option<SubTypeDecl>)> {
    tuple((opt(supertype_constraint), opt(subtype_declaration))).parse(input)
}

/// 318 subtype_declaration = SUBTYPE OF `(` [entity_ref] { `,` [entity_ref] } `)` .
pub fn subtype_declaration(input: &str) -> ParseResult<SubTypeDecl> {
    tuple((
        tag("SUBTYPE"),
        tag("OF"),
        char('('),
        comma_separated(entity_ref),
        char(')'),
    ))
    .map(|(_subtype, _of, _open, entity_references, _close)| SubTypeDecl { entity_references })
    .parse(input)
}

/// 313 subtype_constraint = OF `(` [supertype_expression] `)` .
pub fn subtype_constraint(input: &str) -> ParseResult<SuperTypeExpression> {
    tuple((tag("OF"), char('('), supertype_expression, char(')')))
        .map(|(_of, _open, expr, _close)| expr)
        .parse(input)
}

/// 319 supertype_constraint = [abstract_entity_declaration] | [abstract_supertype_declaration] | [supertype_rule] .
pub fn supertype_constraint(input: &str) -> ParseResult<Constraint> {
    alt((
        abstract_supertype_declaration,
        abstract_entity_declaration,
        supertype_rule,
    ))
    .parse(input)
}

/// 320 supertype_expression = [supertype_factor] { ANDOR [supertype_factor] } .
pub fn supertype_expression(input: &str) -> ParseResult<SuperTypeExpression> {
    tuple((
        supertype_factor,
        many0(tuple((tag("ANDOR"), supertype_factor))),
    ))
    .map(|(first, tails)| {
        if tails.len() > 0 {
            let mut factors = vec![first];
            for (_andor, factor) in tails {
                factors.push(factor)
            }
            SuperTypeExpression::AndOr { factors }
        } else {
            first
        }
    })
    .parse(input)
}

/// 321 supertype_factor = [supertype_term] { AND [supertype_term] } .
pub fn supertype_factor(input: &str) -> ParseResult<SuperTypeExpression> {
    tuple((supertype_term, many0(tuple((tag("AND"), supertype_term)))))
        .map(|(first, tails)| {
            if tails.len() > 0 {
                let mut terms = vec![first];
                for (_and, term) in tails {
                    terms.push(term)
                }
                SuperTypeExpression::And { terms }
            } else {
                first
            }
        })
        .parse(input)
}

/// 323 supertype_term = [entity_ref] | [one_of] | `(` [supertype_expression] `)` .
pub fn supertype_term(input: &str) -> ParseResult<SuperTypeExpression> {
    let expr =
        tuple((char('('), supertype_expression, char(')'))).map(|(_open, expr, _close)| expr);
    alt((
        entity_ref.map(|e| SuperTypeExpression::Reference(e)),
        one_of,
        expr,
    ))
    .parse(input)
}

/// 322 supertype_rule = SUPERTYPE [subtype_constraint] .
pub fn supertype_rule(input: &str) -> ParseResult<Constraint> {
    tuple((tag("SUPERTYPE"), subtype_constraint))
        .map(|(_supertype, constraint)| Constraint::SuperTypeRule(constraint))
        .parse(input)
}

/// 263 one_of = ONEOF `(` [supertype_expression] { `,` [supertype_expression] } `)` .
pub fn one_of(input: &str) -> ParseResult<SuperTypeExpression> {
    tuple((
        tag("ONEOF"),
        char('('),
        comma_separated(supertype_expression),
        char(')'),
    ))
    .map(|(_oneof, _open, exprs, _close)| SuperTypeExpression::OneOf { exprs })
    .parse(input)
}

/// 315 subtype_constraint_decl = [subtype_constraint_head] [subtype_constraint_body] END_SUBTYPE_CONSTRAINT `;` .
pub fn subtype_constraint_decl(input: &str) -> ParseResult<SubTypeConstraint> {
    tuple((
        subtype_constraint_head,
        subtype_constraint_body,
        tag("END_SUBTYPE_CONSTRAINT"),
        char(';'),
    ))
    .map(
        |((name, entity), (is_abstract, total_over, expr), _end, _semicolon)| SubTypeConstraint {
            name,
            entity,
            is_abstract,
            total_over,
            expr,
        },
    )
    .parse(input)
}

/// 316 subtype_constraint_head = SUBTYPE_CONSTRAINT [subtype_constraint_id] FOR [entity_ref] `;` .
pub fn subtype_constraint_head(input: &str) -> ParseResult<(String, String)> {
    tuple((
        tag("SUBTYPE_CONSTRAINT"),
        subtype_constraint_id,
        tag("FOR"),
        entity_ref,
        char(';'),
    ))
    .map(|(_start, id, _for, entity, _semicolon)| (id, entity))
    .parse(input)
}

/// 314 subtype_constraint_body = \[ [abstract_supertype] \] \[ [total_over] \] \[ [supertype_expression] `;` \] .
pub fn subtype_constraint_body(
    input: &str,
) -> ParseResult<(bool, Option<Vec<String>>, Option<SuperTypeExpression>)> {
    tuple((
        opt(abstract_supertype).map(|opt| opt.is_some()),
        opt(total_over),
        opt(supertype_expression),
    ))
    .parse(input)
}

/// 326 total_over = TOTAL_OVER `(` [entity_ref] { `,` [entity_ref] } `)` `;` .
pub fn total_over(input: &str) -> ParseResult<Vec<String>> {
    tuple((
        tag("TOTAL_OVER"),
        char('('),
        many1(entity_ref),
        char(')'),
        char(';'),
    ))
    .map(|(_start, _open, references, _close, _semicolon)| references)
    .parse(input)
}

/// 165 abstract_supertype = ABSTRACT SUPERTYPE `;` .
pub fn abstract_supertype(input: &str) -> ParseResult<()> {
    tuple((tag("ABSTRACT"), tag("SUPERTYPE"), char(';')))
        .map(|(_abstract, _supertype, _semicolon)| ())
        .parse(input)
}
