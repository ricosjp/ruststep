use crate::parser::{combinator::*, token::*, value::*};
use inflector::Inflector;
use nom::{branch::alt, combinator::value, Parser};
use serde::{de, forward_to_deserialize_any, Deserialize};

/// Primitive value type in STEP data, parsed by [parameter]
///
/// Parse
/// ------
///
/// ```
/// use nom::Finish;
/// use ruststep::parser::{Parameter, exchange};
///
/// // Real number
/// let (residual, p) = exchange::parameter("1.0").finish().unwrap();
/// assert_eq!(residual, "");
/// assert_eq!(p, Parameter::real(1.0));
///
/// // String
/// let (residual, p) = exchange::parameter("'ruststep'").finish().unwrap();
/// assert_eq!(residual, "");
/// assert_eq!(p, Parameter::string("ruststep"));
///
/// // non-uniform list
/// let (residual, p) = exchange::parameter("('ruststep', 1.0)").finish().unwrap();
/// assert_eq!(residual, "");
/// assert_eq!(p, [Parameter::string("ruststep"), Parameter::real(1.0)].iter().collect());
///
/// // inline typed struct
/// let (residual, p) = exchange::parameter("FILE_NAME('ruststep')").finish().unwrap();
/// assert_eq!(residual, "");
/// assert!(matches!(p, Parameter::Typed { .. }));
///
/// // inline struct or list can be nested, i.e. `Parameter` can be a tree.
/// let (residual, p) = exchange::parameter("B((1.0, A((2.0, 3.0))))").finish().unwrap();
/// assert_eq!(residual, "");
/// if let Parameter::Typed { name, ty } = p {
///     assert_eq!(name, "B");
///     if let Parameter::List(parameters) = *ty {
///         assert_eq!(parameters.len(), 2);
///         assert_eq!(parameters[0], Parameter::real(1.0));
///         if let Parameter::Typed { name, ty } = &parameters[1] {
///             assert_eq!(name, "A");
///             if let Parameter::List(inner) = &**ty {
///                 assert_eq!(inner.len(), 2);
///                 assert_eq!(inner[0], Parameter::real(2.0));
///                 assert_eq!(inner[1], Parameter::real(3.0));
///             }
///         } else {
///             unreachable!()
///         }
///     } else {
///         unreachable!()
///     }
/// } else {
///     unreachable!()
/// }
/// ```
///
/// FromIterator
/// -------------
/// Create a list as `Parameter::List` from `Iterator<Item=Parameter>` or `Iterator<Item=&Parameter>`.
///
/// ```
/// use ruststep::parser::Parameter;
///
/// let p: Parameter = [Parameter::real(1.0), Parameter::real(2.0)]
///     .iter()
///     .collect();
/// assert!(matches!(p, Parameter::List(_)));
/// ```
///
/// serde::Deserializer
/// -------------------
///
/// This implements a [serde::Deserializer], i.e. a **data format**.
///
/// - For untyped parameters, e.g. real number, can be deserialized into any types
///   as far as compatible in terms of the serde data model.
///
/// ```
/// use serde::Deserialize;
/// use ruststep::parser::Parameter;
///
/// #[derive(Debug, Deserialize)]
/// struct A {
///     x: f64,
///     y: f64,
/// }
///
/// // Create a list as `Parameter::List`
/// let p: Parameter = [Parameter::real(1.0), Parameter::real(2.0)]
///     .iter()
///     .collect();
///
/// // Deserialize the `Parameter` sequence into `A`
/// let a: A = Deserialize::deserialize(&p).unwrap();
/// println!("{:?}", a);
///
/// // Input types will be checked at runtime:
/// let p: Parameter = [Parameter::string("a"), Parameter::integer(2)]
///     .iter()
///     .collect();
/// let result: Result<A, _> = Deserialize::deserialize(&p);
/// assert!(result.is_err());
/// ```
///
/// - Typed parameter, e.g. `A(1)`, must be deserialized into a struct
///   whose name is "A".
///
/// ```
/// use serde::Deserialize;
/// use ruststep::parser::{Parameter, exchange};
/// use nom::Finish;
///
/// #[derive(Debug, Deserialize)]
/// struct A {
///     x: f64,
///     y: f64,
/// }
///
/// // can be deserialized into `A`
/// let (res, p) = exchange::parameter("A((1.0, 2.0))").finish().unwrap();
/// assert_eq!(res, "");
/// let a: A = Deserialize::deserialize(&p).unwrap();
///
/// // B(...) cannot be parsed as `A`
/// let (res, p) = exchange::parameter("B((1.0, 2.0))").finish().unwrap();
/// assert_eq!(res, "");
/// let a: Result<A, _> = Deserialize::deserialize(&p);
/// assert!(a.is_err());
/// ```
///
/// - For [RValue]
///
/// ```
/// use serde::Deserialize;
/// use ruststep::parser::{exchange, value::RValue, Parameter};
/// use nom::Finish;
///
/// let (res, p) = exchange::parameter("#11").finish().unwrap();
/// let a: RValue = Deserialize::deserialize(&p).unwrap();
/// assert_eq!(a, RValue::Entity(11))
/// ```
///
/// [serde::Deserializer]: https://docs.serde.rs/serde/trait.Deserializer.html
///
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Parameter {
    /// Inline *Typed* struct
    Typed { name: String, ty: Box<Parameter> },

    /// Signed integer
    Integer(i64),
    /// Real number
    Real(f64),
    /// string literal
    String(String),
    /// Enumeration defined in EXPRESS schema, like `.TRUE.`
    Enumeration(String),
    /// List of other parameters
    List(Vec<Parameter>),

    /// A reference to entity or value, parsed by [rhs_occurrence_name]
    RValue(RValue),

    /// The special token dollar sign (`$`) is used to represent an object whose value is not provided in the exchange structure.
    NotProvided,
    /// Omitted parameter denoted by `*`
    Omitted,
}

impl Parameter {
    pub fn integer(i: i64) -> Self {
        Parameter::Integer(i)
    }

    pub fn real(x: f64) -> Self {
        Parameter::Real(x)
    }

    pub fn string(s: &str) -> Self {
        Parameter::String(s.to_string())
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
        Parameter::String(value)
    }
}

impl std::iter::FromIterator<Parameter> for Parameter {
    fn from_iter<Iter: IntoIterator<Item = Parameter>>(iter: Iter) -> Self {
        Parameter::List(iter.into_iter().collect())
    }
}

impl<'a> std::iter::FromIterator<&'a Parameter> for Parameter {
    fn from_iter<Iter: IntoIterator<Item = &'a Parameter>>(iter: Iter) -> Self {
        iter.into_iter().cloned().collect()
    }
}

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
        .map(|(name, _open, ty, _close)| Parameter::Typed {
            name,
            ty: Box::new(ty),
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
        rhs_occurrence_name.map(Parameter::RValue),
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

impl<'de, 'param> de::Deserializer<'de> for &'param Parameter {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Parameter::Typed { .. } => unimplemented!(),
            Parameter::Integer(val) => visitor.visit_i64(*val),
            Parameter::Real(val) => visitor.visit_f64(*val),
            Parameter::String(val) => visitor.visit_str(val),
            Parameter::List(params) => {
                visitor.visit_seq(de::value::SeqDeserializer::new(params.iter()))
            }
            Parameter::RValue(rvalue) => de::Deserializer::deserialize_any(rvalue, visitor),
            Parameter::NotProvided | Parameter::Omitted => visitor.visit_none(),
            Parameter::Enumeration(_) => unimplemented!(),
        }
    }

    fn deserialize_struct<V>(
        self,
        struct_name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if let Parameter::Typed { name, ty } = self {
            if struct_name != name.to_pascal_case() {
                return Err(de::Error::invalid_type(
                    de::Unexpected::Other(name),
                    &struct_name,
                ));
            }
            ty.deserialize_any(visitor)
        } else {
            self.deserialize_any(visitor)
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum identifier ignored_any
    }
}

impl<'de, 'param> de::IntoDeserializer<'de, crate::error::Error> for &'param Parameter {
    type Deserializer = Self;
    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Finish;
    use serde::Deserialize;

    #[test]
    fn deserialize_int() {
        let p = Parameter::Integer(2);
        let a: i64 = Deserialize::deserialize(&p).unwrap();
        assert_eq!(a, 2);
        // can be deserialized as unsigned
        let a: u32 = Deserialize::deserialize(&p).unwrap();
        assert_eq!(a, 2);

        let p = Parameter::Integer(-2);
        let a: i64 = Deserialize::deserialize(&p).unwrap();
        assert_eq!(a, -2);
        // cannot be deserialized negative integer into unsigned
        let res: Result<u32, _> = Deserialize::deserialize(&p);
        assert!(res.is_err());
    }

    #[derive(Debug, Deserialize)]
    struct A {
        x: f64,
        y: f64,
    }

    #[derive(Debug, Deserialize)]
    struct B {
        z: f64,
        a: A,
    }

    #[test]
    fn deserialize_parameter_typed_nested() {
        let (res, p) = super::parameter("B((1.0, A((2.0, 3.0))))")
            .finish()
            .unwrap();
        assert_eq!(res, "");
        let b: B = Deserialize::deserialize(dbg!(&p)).unwrap();
        dbg!(b);

        // C(...) should not be parsed as A
        let (res, p) = super::parameter("B((1.0, C((2.0, 3.0))))")
            .finish()
            .unwrap();
        assert_eq!(res, "");
        let b: Result<B, _> = Deserialize::deserialize(dbg!(&p));
        assert!(b.is_err());
    }
}
