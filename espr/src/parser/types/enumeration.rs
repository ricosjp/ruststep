use super::{
    super::{basis::*, util::*},
    *,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enumeration {
    pub extensiblity: Extensiblity,
    pub items: Vec<String>,
}

/// 211 enumeration_items = `(` enumeration_id { `,` enumeration_id } `)` .
pub fn enumeration_items(input: &str) -> ParseResult<Vec<String>> {
    tuple((char('('), comma_separated(remarked(simple_id)), char(')')))
        .map(|(_open, enums, _close)| enums)
        .parse(input)
}

/// 213 enumeration_type = [ EXTENSIBLE ] ENUMERATION [ ( OF enumeration_items ) | enumeration_extension ] .
pub fn enumeration_type(input: &str) -> ParseResult<Enumeration> {
    // FIXME enumeration_extension
    tuple((
        opt(tag("EXTENSIBLE")),
        tag("ENUMERATION"),
        tag("OF"),
        enumeration_items,
    ))
    .map(|(extensiblility, _start, _of, items)| Enumeration {
        extensiblity: if extensiblility.is_some() {
            Extensiblity::Extensible
        } else {
            Extensiblity::None
        },
        items,
    })
    .parse(input)
}
