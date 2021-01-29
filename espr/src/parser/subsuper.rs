use super::{identifier::*, util::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint {
    AbstractEntity,
    AbstractSuperType(Option<SuperTypeExpression>),
    SuperTypeRule(SuperTypeExpression),
}

/// 164 abstract_entity_declaration = ABSTRACT .
pub fn abstract_entity_declaration(input: &str) -> ParseResult<Constraint> {
    tag("ABSTRACT")
        .map(|_| Constraint::AbstractEntity)
        .parse(input)
}

/// 165 abstract_supertype = ABSTRACT SUPERTYPE ’;’ .
pub fn abstract_supertype(input: &str) -> ParseResult<()> {
    tuple((tag("ABSTRACT"), tag("SUPERTYPE"), char(';')))
        .map(|(_abstract, _supertype, _semicoron)| ())
        .parse(input)
}

/// 166 abstract_supertype_declaration = ABSTRACT SUPERTYPE [ subtype_constraint ] .
pub fn abstract_supertype_declaration(input: &str) -> ParseResult<Constraint> {
    tuple((tag("ABSTRACT"), tag("SUPERTYPE"), opt(subtype_constraint)))
        .map(|(_abstract, _supertype, expr)| Constraint::AbstractSuperType(expr))
        .parse(input)
}

/// 312 subsuper = [ supertype_constraint ] [ subtype_declaration ] .
pub fn subsuper(input: &str) -> ParseResult<(Option<Constraint>, Option<SubTypeDecl>)> {
    tuple((opt(supertype_constraint), opt(subtype_declaration))).parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubTypeDecl {
    pub entity_references: Vec<String>,
}

/// 318 subtype_declaration = SUBTYPE OF ’(’ entity_ref { ’,’ entity_ref } ’)’ .
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

/// 313 subtype_constraint = OF ’(’ supertype_expression ’)’ .
pub fn subtype_constraint(input: &str) -> ParseResult<SuperTypeExpression> {
    tuple((tag("OF"), char('('), supertype_expression, char(')')))
        .map(|(_of, _open, expr, _close)| expr)
        .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuperTypeExpression {
    Reference(String),
    AndOr { factors: Vec<SuperTypeExpression> },
    And { terms: Vec<SuperTypeExpression> },
    OneOf { exprs: Vec<SuperTypeExpression> },
}

/// 319 supertype_constraint = abstract_entity_declaration | abstract_supertype_declaration | supertype_rule .
pub fn supertype_constraint(input: &str) -> ParseResult<Constraint> {
    alt((
        abstract_supertype_declaration,
        abstract_entity_declaration,
        supertype_rule,
    ))
    .parse(input)
}

/// 320 supertype_expression = supertype_factor { ANDOR supertype_factor } .
pub fn supertype_expression(input: &str) -> ParseResult<SuperTypeExpression> {
    tuple((
        supertype_factor,
        opt(spaced_many0(tuple((tag("ANDOR"), supertype_factor)))),
    ))
    .map(|(first, tails)| {
        let mut factors = vec![first];
        if let Some(tails) = tails {
            for (_andor, factor) in tails {
                factors.push(factor)
            }
        }
        SuperTypeExpression::AndOr { factors }
    })
    .parse(input)
}

/// 321 supertype_factor = supertype_term { AND supertype_term } .
pub fn supertype_factor(input: &str) -> ParseResult<SuperTypeExpression> {
    tuple((
        supertype_term,
        opt(spaced_many0(tuple((tag("AND"), supertype_term)))),
    ))
    .map(|(first, tails)| {
        let mut terms = vec![first];
        if let Some(tails) = tails {
            for (_and, term) in tails {
                terms.push(term)
            }
        }
        SuperTypeExpression::And { terms }
    })
    .parse(input)
}

/// 323 supertype_term = entity_ref | one_of | `(` supertype_expression `)` .
pub fn supertype_term(input: &str) -> ParseResult<SuperTypeExpression> {
    alt((
        one_of,
        supertype_expression,
        entity_ref.map(|e| SuperTypeExpression::Reference(e)),
    ))
    .parse(input)
}

/// 322 supertype_rule = SUPERTYPE subtype_constraint .
pub fn supertype_rule(input: &str) -> ParseResult<Constraint> {
    tuple((tag("SUPERTYPE"), subtype_constraint))
        .map(|(_supertype, constraint)| Constraint::SuperTypeRule(constraint))
        .parse(input)
}

/// 263 one_of = ONEOF `(` supertype_expression { `,` supertype_expression } `)` .
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
