use crate::parser::{combinator::*, exchange::*, token::*};

/// data_section = `DATA` \[ `(` [parameter_list] `)` \] `;` [entity_instance_list] `ENDSEC;` .
pub fn data_section(input: &str) -> ParseResult<()> {
    todo!()
}

/// entity_instance_list = { [entity_instance] } .
pub fn entity_instance_list(input: &str) -> ParseResult<()> {
    todo!()
}

/// entity_instance = [simple_entity_instance] | [complex_entity_instance] .
pub fn entity_instance(input: &str) -> ParseResult<()> {
    todo!()
}

/// simple_entity_instance = [entity_instance_name] `=` [simple_record] `;` .
pub fn simple_entity_instance(input: &str) -> ParseResult<()> {
    todo!()
}

/// complex_entity_instance = [entity_instance_name] `=` [subsuper_record] `;` .
pub fn complex_entity_instance(input: &str) -> ParseResult<()> {
    todo!()
}

/// simple_record = [keyword] `(` \[ [parameter_list] \] `)` .
pub fn simple_record(input: &str) -> ParseResult<()> {
    todo!()
}

/// subsuper_record = `(` [simple_record_list] `)` .
pub fn subsuper_record(input: &str) -> ParseResult<()> {
    todo!()
}

/// simple_record_list = [simple_record] { [simple_record] } .
pub fn simple_record_list(input: &str) -> ParseResult<()> {
    todo!()
}
