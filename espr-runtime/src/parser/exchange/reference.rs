use crate::parser::combinator::*;

/// reference_section = `REFERENCE;` [reference_list] `ENDSEC;` .
pub fn reference_section(input: &str) -> ParseResult<()> {
    todo!()
}

/// reference_list = { [reference] } .
pub fn reference_list(input: &str) -> ParseResult<()> {
    todo!()
}

/// reference = [lhs_occurrence_name] `=` [resource] `;` .
pub fn reference(input: &str) -> ParseResult<()> {
    todo!()
}
