use super::{
    super::{basis::*, util::*},
    *,
};

#[derive(Debug, Clone, PartialEq)]
pub enum QualifiableFactor {
    /// `attribute_ref` or `general_ref` or `population`
    Reference(String),
}

/// 274 qualifiable_factor = attribute_ref | constant_factor | function_call | general_ref | population .
pub fn qualifiable_factor(input: &str) -> ParseResult<QualifiableFactor> {
    // FIXME support constant_factor
    // FIXME support function_call
    remarked(simple_id)
        .map(|id| QualifiableFactor::Reference(id))
        .parse(input)
}

/// Output of [qualifier]
///
/// `SELF\point.x`
#[derive(Debug, Clone, PartialEq)]
pub enum Qualifier {
    /// Like `.x`
    Attribute(String),
    /// Like `\point`
    Group(String),
    Index {
        index_1: SimpleExpression,
        index_2: Option<SimpleExpression>,
    },
}

/// 276 qualifier = attribute_qualifier | group_qualifier | index_qualifier .
pub fn qualifier(input: &str) -> ParseResult<Qualifier> {
    alt((attribute_qualifier, group_qualifier, index_qualifier)).parse(input)
}

/// 179 attribute_qualifier = `.` attribute_ref .
pub fn attribute_qualifier(input: &str) -> ParseResult<Qualifier> {
    tuple((char('.'), remarked(simple_id)))
        .map(|(_dot, id)| Qualifier::Attribute(id))
        .parse(input)
}

/// 232 group_qualifier = `\` entity_ref .
pub fn group_qualifier(input: &str) -> ParseResult<Qualifier> {
    tuple((char('\\'), remarked(simple_id)))
        .map(|(_dot, id)| Qualifier::Group(id))
        .parse(input)
}

/// 239 index_qualifier = `[` index_1 [ `:` index_2 ] `]` .
pub fn index_qualifier(input: &str) -> ParseResult<Qualifier> {
    tuple((
        char('['),
        simple_expression,
        opt(tuple((char(':'), simple_expression))),
        char(']'),
    ))
    .map(|(_open, index_1, opt, _close)| Qualifier::Index {
        index_1,
        index_2: opt.map(|(_coron, expr)| expr),
    })
    .parse(input)
}
