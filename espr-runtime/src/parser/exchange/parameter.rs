use crate::parser::{combinator::*, token::*};
use nom::{branch::alt, combinator::value, Parser};

#[derive(Debug, Clone, PartialEq)]
pub enum UntypedParameter {
    Integer(i64),
    Real(f64),
    String(String),
    Enumeration(String),
    List(Vec<Parameter>),
    // FIXME Add Binary, $
}

/// list = `(` \[ [parameter] { `,` [parameter] } \] `)` .
pub fn list(input: &str) -> ParseResult<UntypedParameter> {
    tuple_((char_('('), comma_separated(parameter), char_(')')))
        .map(|(_open, params, _close)| UntypedParameter::List(params))
        .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Parameter {
    Typed {
        name: String,
        ty: Box<Parameter>,
    },
    Untyped(UntypedParameter),
    /// `*`
    Omitted,
}

/// parameter = [typed_parameter] | [untyped_parameter] | [omitted_parameter] .
pub fn parameter(input: &str) -> ParseResult<Parameter> {
    alt((typed_parameter, untyped_parameter, omitted_parameter)).parse(input)
}

/// typed_parameter = [keyword] `(` [parameter] `)` .
pub fn typed_parameter(input: &str) -> ParseResult<Parameter> {
    tuple_((keyword, char_('('), parameter, char_(')')))
        .map(|(name, _open, ty, _close)| Parameter::Typed {
            name,
            ty: Box::new(ty),
        })
        .parse(input)
}

/// untyped_parameter = `$` | [integer] | [real] | [string] | [rhs_occurence_name] | [enumeration] | [binary] | [list] .
pub fn untyped_parameter(input: &str) -> ParseResult<Parameter> {
    alt((
        // FIXME `$`
        integer.map(|val| UntypedParameter::Integer(val)),
        real.map(|val| UntypedParameter::Real(val)),
        string.map(|val| UntypedParameter::String(val)),
        // FIXME rhs_occurence_name
        enumeration.map(|val| UntypedParameter::Enumeration(val)),
        // FIXME binary
        list,
    ))
    .map(|untyped| Parameter::Untyped(untyped))
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
