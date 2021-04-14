use crate::{
    parser::{combinator::*, token::*},
    step::*,
};
use nom::Parser;

/// reference_section = `REFERENCE;` [reference_list] `ENDSEC;` .
pub fn reference_section(input: &str) -> ParseResult<Vec<ReferenceEntry>> {
    tuple_((tag_("REFERENCE;"), reference_list, tag_("ENDSEC;")))
        .map(|(_start, list, _end)| list)
        .parse(input)
}

/// reference_list = { [reference()] } .
pub fn reference_list(input: &str) -> ParseResult<Vec<ReferenceEntry>> {
    many0_(reference).parse(input)
}

/// reference = [lhs_occurrence_name] `=` [resource] `;` .
pub fn reference(input: &str) -> ParseResult<ReferenceEntry> {
    tuple_((lhs_occurrence_name, char_('='), resource, char_(';')))
        .map(|(name, _def, resource, _semicolon)| ReferenceEntry { name, resource })
        .parse(input)
}
