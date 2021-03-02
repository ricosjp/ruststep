use crate::parser::{combinator::*, token::*};
use nom::{branch::alt, Parser};

/// anchor_section = `ANCHOR;` [anchor_list] `ENDSEC;` .
pub fn anchor_section(input: &str) -> ParseResult<Vec<Anchor>> {
    tuple_((tag_("ANCHOR;"), anchor_list, tag_("ENDSEC;")))
        .map(|(_start, anchors, _end)| anchors)
        .parse(input)
}

/// anchor_list = { [anchor()] } .
pub fn anchor_list(input: &str) -> ParseResult<Vec<Anchor>> {
    many0_(anchor).parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Anchor {
    pub name: String,
    pub item: AnchorItem,
    pub tags: Vec<(String, AnchorItem)>,
}

/// anchor = [anchor_name] `=` [anchor_item] { [anchor_tag] } `;` .
pub fn anchor(input: &str) -> ParseResult<Anchor> {
    tuple_((
        anchor_name,
        char_('='),
        anchor_item,
        many0_(anchor_tag),
        char_(';'),
    ))
    .map(|(name, _eq, item, tags, _semicolon)| Anchor { name, item, tags })
    .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnchorItem {
    Integer(i64),
    Real(f64),
    String(String),
    Enumeration(String),
    /// The special token dollar sign (`$`) is used to represent an object whose value is not provided in the exchange structure.
    NotProvided,
    /// A reference to entity or value, parsed by [rhs_occurrence_name]
    RValue(RValue),
    /// List of other parameters
    List(Vec<AnchorItem>),
}

/// anchor_item = `$` | [integer] | [real] | [string] | [enumeration] | binary | [rhs_occurrence_name] | [resource] | [anchor_item_list] .
pub fn anchor_item(input: &str) -> ParseResult<AnchorItem> {
    alt((
        char_('$').map(|_| AnchorItem::NotProvided),
        integer.map(AnchorItem::Integer),
        real.map(AnchorItem::Real),
        string.map(AnchorItem::String),
        rhs_occurrence_name.map(AnchorItem::RValue),
        enumeration.map(AnchorItem::Enumeration),
        // FIXME binary
        anchor_item_list,
    ))
    .parse(input)
}

/// anchor_item_list = `(` \[ [anchor_item] { `,` [anchor_item] } \] `)` .
pub fn anchor_item_list(input: &str) -> ParseResult<AnchorItem> {
    tuple_((char_('('), opt_(comma_separated(anchor_item)), char_(')')))
        .map(|(_open, anchors, _close)| AnchorItem::List(anchors.unwrap_or_default()))
        .parse(input)
}

/// anchor_tag = `{` [tag_name] `:` [anchor_item] `}` .
pub fn anchor_tag(input: &str) -> ParseResult<(String, AnchorItem)> {
    tuple_((char_('{'), tag_name, char_(':'), anchor_item, char_('}')))
        .map(|(_open, name, _colon, item, _close)| (name, item))
        .parse(input)
}
