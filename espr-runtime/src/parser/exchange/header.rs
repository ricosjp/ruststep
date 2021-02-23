use crate::parser::{combinator::*, exchange::*, token::*};
use nom::Parser;

/// header_section = `HEADER;` [header_entity] [header_entity] [header_entity] \[ [header_entity_list] \] `ENDSEC;` .
pub fn header_section(input: &str) -> ParseResult<()> {
    todo!()
}

/// header_entity_list = [header_entity] { [header_entity] } .
pub fn header_entity_list(input: &str) -> ParseResult<Vec<HeaderEntity>> {
    many1_(header_entity).parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeaderEntity {
    pub name: String,
    pub parameters: Vec<Parameter>,
}

/// header_entity = [keyword] `(` \[ [parameter_list] \] `)` `;` .
pub fn header_entity(input: &str) -> ParseResult<HeaderEntity> {
    tuple_((keyword, char_('('), opt_(parameter_list), char_(')')))
        .map(|(name, _open, parameters, _close)| HeaderEntity {
            name,
            parameters: parameters.unwrap_or_default(),
        })
        .parse(input)
}
