use crate::parser::combinator::*;

/// HEADER_SECTION = "HEADER;" HEADER_ENTITY HEADER_ENTITY HEADER_ENTITY [HEADER_ENTITY_LIST] "ENDSEC;" .
pub fn header_section(input: &str) -> ParseResult<()> {
    todo!()
}

/// HEADER_ENTITY_LIST = HEADER_ENTITY { HEADER_ENTITY } .
pub fn header_entity_list(input: &str) -> ParseResult<()> {
    todo!()
}

/// HEADER_ENTITY = KEYWORD  "(" [ PARAMETER_LIST ] ")" ";" .
pub fn header_entity(input: &str) -> ParseResult<()> {
    todo!()
}
