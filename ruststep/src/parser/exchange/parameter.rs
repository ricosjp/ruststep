use crate::{
    ast::*,
    parser::{combinator::*, token::*},
};
use nom::{branch::alt, combinator::value, Parser};

/// list = `(` \[ [parameter] { `,` [parameter] } \] `)` .
pub fn list(input: &str) -> ParseResult<Parameter> {
    tuple_((char_('('), comma_separated(parameter), char_(')')))
        .map(|(_open, params, _close)| Parameter::List(params))
        .parse(input)
}

/// parameter = [typed_parameter] | [untyped_parameter] | [omitted_parameter] .
pub fn parameter(input: &str) -> ParseResult<Parameter> {
    alt((typed_parameter, untyped_parameter, omitted_parameter)).parse(input)
}

/// typed_parameter = [keyword] `(` [parameter] `)` .
pub fn typed_parameter(input: &str) -> ParseResult<Parameter> {
    tuple_((keyword, char_('('), parameter, char_(')')))
        .map(|(name, _open, ty, _close)| {
            Parameter::Typed(SimpleEntityInstance {
                name,
                parameter: Box::new(ty),
            })
        })
        .parse(input)
}

/// untyped_parameter = `$` | [integer] | [real] | [string] | [rhs_occurrence_name] | [enumeration] | binary | [list] .
pub fn untyped_parameter(input: &str) -> ParseResult<Parameter> {
    alt((
        char_('$').map(|_| Parameter::NotProvided),
        real.map(Parameter::Real),
        integer.map(Parameter::Integer),
        string.map(Parameter::String),
        rhs_occurrence_name.map(Parameter::Ref),
        enumeration.map(Parameter::Enumeration),
        // FIXME binary
        list,
    ))
    .parse(input)
}

/// omitted_parameter = `*` .
pub fn omitted_parameter(input: &str) -> ParseResult<Parameter> {
    value(Parameter::Omitted, char_('*')).parse(input)
}

/// parameter_list = [parameter] { `,` [parameter] } .
pub fn parameter_list(input: &str) -> ParseResult<Vec<Parameter>> {
    comma_separated(parameter).parse(input)
}

#[cfg(test)]
mod tests {
    use crate::ast::Parameter;
    use nom::Finish;

    #[test]
    fn untyped_parameter() {
        let (res, record) = super::untyped_parameter("2").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(record, Parameter::integer(2));

        let (res, record) = super::untyped_parameter("2.0").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(record, Parameter::real(2.0));
    }
}
