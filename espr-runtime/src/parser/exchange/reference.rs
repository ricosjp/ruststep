use crate::parser::{combinator::*, token::*};
use nom::Parser;

/// reference_section = `REFERENCE;` [reference_list] `ENDSEC;` .
pub fn reference_section(input: &str) -> ParseResult<Vec<Reference>> {
    tuple_((tag_("REFERENCE;"), reference_list, tag_("ENDSEC;")))
        .map(|(_start, list, _end)| list)
        .parse(input)
}

/// reference_list = { [reference] } .
pub fn reference_list(input: &str) -> ParseResult<Vec<Reference>> {
    many0_(reference).parse(input)
}

pub struct Reference {
    pub name: String,
    pub resource: String,
}

/// reference = [lhs_occurrence_name] `=` [resource] `;` .
pub fn reference(input: &str) -> ParseResult<Reference> {
    tuple_((lhs_occurrence_name, char_('='), resource, char_(';')))
        .map(|(name, _def, resource, _semicolon)| Reference { name, resource })
        .parse(input)
}
