use crate::parser::{combinator::*, token::*};
use nom::{branch::alt, combinator::value, Parser};
use serde::{de, forward_to_deserialize_any};

#[derive(Debug, Clone, PartialEq)]
pub enum UntypedParameter {
    Integer(i64),
    Real(f64),
    String(String),
    Enumeration(String),
    /// The special token dollar sign (`$`) is used to represent an object whose value is not provided in the exchange structure.
    NotProvided,
    /// A reference to entity or value, parsed by [rhs_occurrence_name]
    RValue(RValue),
    /// List of other parameters
    List(Vec<Parameter>),
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

impl Parameter {
    pub fn integer(i: i64) -> Self {
        Parameter::Untyped(UntypedParameter::Integer(i))
    }

    pub fn real(x: f64) -> Self {
        Parameter::Untyped(UntypedParameter::Real(x))
    }

    pub fn string(s: &str) -> Self {
        Parameter::Untyped(UntypedParameter::String(s.to_string()))
    }
}

impl From<i64> for Parameter {
    fn from(value: i64) -> Self {
        Self::integer(value)
    }
}

impl From<f64> for Parameter {
    fn from(value: f64) -> Self {
        Self::real(value)
    }
}

impl From<String> for Parameter {
    fn from(value: String) -> Self {
        Parameter::Untyped(UntypedParameter::String(value))
    }
}

impl std::iter::FromIterator<Parameter> for Parameter {
    fn from_iter<Iter: IntoIterator<Item = Parameter>>(iter: Iter) -> Self {
        Parameter::Untyped(UntypedParameter::List(iter.into_iter().collect()))
    }
}

impl<'a> std::iter::FromIterator<&'a Parameter> for Parameter {
    fn from_iter<Iter: IntoIterator<Item = &'a Parameter>>(iter: Iter) -> Self {
        iter.into_iter().cloned().collect()
    }
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

/// untyped_parameter = `$` | [integer] | [real] | [string] | [rhs_occurrence_name] | [enumeration] | binary | [list] .
pub fn untyped_parameter(input: &str) -> ParseResult<Parameter> {
    alt((
        char_('$').map(|_| UntypedParameter::NotProvided),
        real.map(UntypedParameter::Real),
        integer.map(UntypedParameter::Integer),
        string.map(UntypedParameter::String),
        rhs_occurrence_name.map(UntypedParameter::RValue),
        enumeration.map(UntypedParameter::Enumeration),
        // FIXME binary
        list,
    ))
    .map(Parameter::Untyped)
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

impl<'de, 'param> de::Deserializer<'de> for &'param Parameter {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Parameter::Typed { name: _, ty: _ } => unimplemented!(),
            Parameter::Untyped(p) => match p {
                UntypedParameter::Integer(val) => visitor.visit_i64(*val),
                UntypedParameter::Real(val) => visitor.visit_f64(*val),
                UntypedParameter::String(val) => visitor.visit_str(val),
                _ => unimplemented!(),
            },
            Parameter::Omitted => unimplemented!(),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn list_from_iter() {
        let l: Parameter = [Parameter::integer(1), Parameter::real(2.0)]
            .iter()
            .collect();
        assert!(matches!(l, Parameter::Untyped(UntypedParameter::List(_))));
    }

    #[test]
    fn deserialize_int() {
        let p = Parameter::Untyped(UntypedParameter::Integer(2));
        let a: i64 = Deserialize::deserialize(&p).unwrap();
        assert_eq!(a, 2);
        // can be deserialized as unsigned
        let a: u32 = Deserialize::deserialize(&p).unwrap();
        assert_eq!(a, 2);

        let p = Parameter::Untyped(UntypedParameter::Integer(-2));
        let a: i64 = Deserialize::deserialize(&p).unwrap();
        assert_eq!(a, -2);
        // cannot be deserialized negative integer into unsigned
        let res: Result<u32, _> = Deserialize::deserialize(&p);
        assert!(res.is_err());
    }
}
