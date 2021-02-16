use crate::parser::combinator::*;

/// DATA_SECTION = "DATA" [ "(" PARAMETER_LIST ")" ] ";" ENTITY_INSTANCE_LIST "ENDSEC;" .
pub fn data_section(input: &str) -> ParseResult<()> {
    todo!()
}

/// ENTITY_INSTANCE_LIST = { ENTITY_INSTANCE } .
pub fn entity_instance_list(input: &str) -> ParseResult<()> {
    todo!()
}

/// ENTITY_INSTANCE = SIMPLE_ENTITY_INSTANCE | COMPLEX_ENTITY_INSTANCE .
pub fn entity_instance(input: &str) -> ParseResult<()> {
    todo!()
}

/// SIMPLE_ENTITY_INSTANCE = ENTITY_INSTANCE_NAME "=" SIMPLE_RECORD ";" .
pub fn simple_entity_instance(input: &str) -> ParseResult<()> {
    todo!()
}

/// COMPLEX_ENTITY_INSTANCE = ENTITY_INSTANCE_NAME "=" SUBSUPER_RECORD ";" .
pub fn complex_entity_instance(input: &str) -> ParseResult<()> {
    todo!()
}

/// SIMPLE_RECORD = KEYWORD "(" [ PARAMETER_LIST ] ")" .
pub fn simple_record(input: &str) -> ParseResult<()> {
    todo!()
}

/// SUBSUPER_RECORD = "(" SIMPLE_RECORD_LIST ")" .
pub fn subsuper_record(input: &str) -> ParseResult<()> {
    todo!()
}

/// SIMPLE_RECORD_LIST = SIMPLE_RECORD { SIMPLE_RECORD } .
pub fn simple_record_list(input: &str) -> ParseResult<()> {
    todo!()
}
