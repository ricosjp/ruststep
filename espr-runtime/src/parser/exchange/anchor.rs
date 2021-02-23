use crate::parser::{combinator::*, exchange::*, token::*};
use nom::{branch::alt, Parser};

/// anchor_section = `ANCHOR;` [anchor_list] `ENDSEC;` .
pub fn anchor_section(input: &str) -> ParseResult<()> {
    todo!()
}

/// anchor_list = { [anchor()] } .
pub fn anchor_list(input: &str) -> ParseResult<()> {
    todo!()
}

/// anchor = [anchor_name] `=` [anchor_item] { [anchor_tag] } `;` .
pub fn anchor(input: &str) -> ParseResult<()> {
    todo!()
}

#[derive(Debug, Clone, PartialEq)]
pub enum Anchor {
    Integer(i64),
    Real(f64),
    String(String),
    Enumeration(String),
    /// The special token dollar sign (`$`) is used to represent an object whose value is not provided in the exchange structure.
    NotProvided,
    /// A reference to entity or value, parsed by [rhs_occurrence_name]
    RValue(RValue),
    /// List of other parameters
    List(Vec<Anchor>),
}

/// anchor_item = `$` | [integer] | [real] | [string] | [enumeration] | binary | [rhs_occurrence_name] | [resource] | [anchor_item_list] .
pub fn anchor_item(input: &str) -> ParseResult<Anchor> {
    alt((
        char_('$').map(|_| Anchor::NotProvided),
        integer.map(|val| Anchor::Integer(val)),
        real.map(|val| Anchor::Real(val)),
        string.map(|val| Anchor::String(val)),
        rhs_occurrence_name.map(|val| Anchor::RValue(val)),
        enumeration.map(|val| Anchor::Enumeration(val)),
        // FIXME binary
        anchor_item_list,
    ))
    .parse(input)
}

/// anchor_item_list = `(` \[ [anchor_item] { `,` [anchor_item] } \] `)` .
pub fn anchor_item_list(input: &str) -> ParseResult<Anchor> {
    tuple_((char_('('), opt_(comma_separated(anchor_item)), char_(')')))
        .map(|(_open, anchors, _close)| Anchor::List(anchors.unwrap_or_default()))
        .parse(input)
}

/// anchor_tag = `{` [tag_name] `:` [anchor_item] `}` .
pub fn anchor_tag(input: &str) -> ParseResult<()> {
    todo!()
}
