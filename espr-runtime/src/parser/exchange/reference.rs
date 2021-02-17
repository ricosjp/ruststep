use crate::parser::combinator::*;

/// REFERENCE_SECTION = `REFERENCE;` REFERENCE_LIST `ENDSEC;` .
pub fn reference_section(input: &str) -> ParseResult<()> {
    todo!()
}

/// REFERENCE_LIST = { REFERENCE } .
pub fn reference_list(input: &str) -> ParseResult<()> {
    todo!()
}

/// REFERENCE = LHS_OCCURRENCE_NAME `=` RESOURCE `;` .
pub fn reference(input: &str) -> ParseResult<()> {
    todo!()
}
