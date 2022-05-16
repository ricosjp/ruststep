use crate::{
    ast::*,
    parser::{combinator::*, exchange::*, token::*},
};
use nom::{branch::alt, Parser};

/// data_section = `DATA` \[ `(` [parameter_list] `)` \] `;` [entity_instance_list] `ENDSEC;` .
pub fn data_section(input: &str) -> ParseResult<DataSection> {
    tuple_((
        tag_("DATA"),
        opt_(tuple_((char_('('), parameter_list, char_(')')))),
        char_(';'),
        entity_instance_list,
        tag_("ENDSEC;"),
    ))
    .map(|(_start, meta, _semicolon, entities, _end)| DataSection {
        meta: meta
            .map(|(_open, params, _close)| params)
            .unwrap_or_default(),
        entities,
    })
    .parse(input)
}

/// entity_instance_list = { [entity_instance] } .
pub fn entity_instance_list(input: &str) -> ParseResult<Vec<EntityInstance>> {
    many0_(entity_instance).parse(input)
}

/// entity_instance = [simple_entity_instance] | [complex_entity_instance] .
pub fn entity_instance(input: &str) -> ParseResult<EntityInstance> {
    alt((simple_entity_instance, complex_entity_instance)).parse(input)
}

/// simple_entity_instance = [entity_instance_name] `=` [simple_record] `;` .
pub fn simple_entity_instance(input: &str) -> ParseResult<EntityInstance> {
    tuple_((entity_instance_name, char_('='), simple_record, char_(';')))
        .map(|(id, _eq, record, _semicolon)| EntityInstance::Simple { id, record })
        .parse(input)
}

/// complex_entity_instance = [entity_instance_name] `=` [subsuper_record] `;` .
pub fn complex_entity_instance(input: &str) -> ParseResult<EntityInstance> {
    tuple_((
        entity_instance_name,
        char_('='),
        subsuper_record,
        char_(';'),
    ))
    .map(|(id, _eq, subsuper, _semicolon)| EntityInstance::Complex { id, subsuper })
    .parse(input)
}

/// simple_record = [keyword] `(` \[ [parameter_list] \] `)` .
pub fn simple_record(input: &str) -> ParseResult<SimpleEntityInstance> {
    tuple_((keyword, char_('('), opt_(parameter_list), char_(')')))
        .map(
            |(keyword, _open, parameters, _close)| SimpleEntityInstance {
                keyword,
                parameters: parameters.unwrap_or_default().into_iter().collect(),
            },
        )
        .parse(input)
}

/// simple_record_list = [simple_record] { [simple_record] } .
pub fn simple_record_list(input: &str) -> ParseResult<Vec<SimpleEntityInstance>> {
    many0_(simple_record).parse(input)
}

/// subsuper_record = `(` [simple_record_list] `)` .
pub fn subsuper_record(input: &str) -> ParseResult<Vec<SimpleEntityInstance>> {
    tuple_((char_('('), simple_record_list, char_(')')))
        .map(|(_open, records, _close)| records)
        .parse(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn simple_recode1() {
        let (res, record) = super::simple_record("A(1, 2.0)").finish().unwrap();
        dbg!(record);
        assert_eq!(res, "");
    }

    #[test]
    fn simple_recode2() {
        let (res, record) = super::simple_record(
            "LENGTH_MEASURE_WITH_UNIT( LENGTH_MEASURE( 1.00000000000000 ), #359 )",
        )
        .finish()
        .unwrap();
        dbg!(record);
        assert_eq!(res, "");
    }
}
