use crate::{
    parser::{combinator::*, exchange::*},
    step::*,
};
use nom::Parser;

/// header_section = `HEADER;` [header_entity] [header_entity] [header_entity] \[ [header_entity_list] \] `ENDSEC;` .
pub fn header_section(input: &str) -> ParseResult<Vec<Record>> {
    tuple_((tag_("HEADER;"), header_entity_list, tag_("ENDSEC;")))
        .map(|(_start, entities, _close)| entities)
        .parse(input)
}

/// header_entity_list = [header_entity] { [header_entity] } .
pub fn header_entity_list(input: &str) -> ParseResult<Vec<Record>> {
    many1_(header_entity).parse(input)
}

/// header_entity = [simple_record] `;` .
///
/// Changed from the following original definition due to the duplication
///
/// ```text
/// header_entity = keyword ( [ parameter_list ] ) ; .
/// ```
pub fn header_entity(input: &str) -> ParseResult<Record> {
    tuple_((simple_record, char_(';')))
        .map(|(record, _semicolon)| record)
        .parse(input)
}
