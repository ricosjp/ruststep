use crate::parser::combinator::*;

/// header_section = `HEADER;` [header_entity] [header_entity] [header_entity] \[ [header_entity_list] \] `ENDSEC;` .
pub fn header_section(input: &str) -> ParseResult<()> {
    todo!()
}

/// header_entity_list = [header_entity] { [header_entity] } .
pub fn header_entity_list(input: &str) -> ParseResult<()> {
    todo!()
}

/// header_entity = [keyword ] `(` \[ [parameter_list] \] `)` `;` .
pub fn header_entity(input: &str) -> ParseResult<()> {
    todo!()
}
