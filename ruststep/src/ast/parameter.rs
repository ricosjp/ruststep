use crate::ast::*;
use serde::{
    de::{self, VariantAccess},
    forward_to_deserialize_any, Deserialize,
};
use std::fmt;

/// Primitive value type in STEP data
///
/// Parse
/// ------
///
/// ```
/// use nom::Finish;
/// use ruststep::{parser::exchange, ast::Parameter};
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
/// ```
///
/// Inline struct or list can be nested, i.e. `Parameter` can be a tree.
///
/// ```
/// use nom::Finish;
/// use ruststep::{parser::exchange, ast::Parameter};
///
/// let (residual, p) = exchange::parameter("B((1.0, A((2.0, 3.0))))")
///     .finish()
///     .unwrap();
/// assert_eq!(residual, "");
///
/// // A((2.0, 3.0))
/// let a = Parameter::Typed {
///     name: "A".to_string(),
///     ty: Box::new(
///         [Parameter::real(2.0), Parameter::real(3.0)]
///             .iter()
///             .collect(),
///     ),
/// };
///
/// // B((1.0, a))
/// let b = Parameter::Typed {
///     name: "B".to_string(),
///     ty: Box::new([Parameter::real(1.0), a].iter().collect()),
/// };
///
/// assert_eq!(p, b);
/// ```
///
/// FromIterator
/// -------------
/// Create a list as `Parameter::List` from `Iterator<Item=Parameter>` or `Iterator<Item=&Parameter>`.
///
/// ```
/// use ruststep::ast::Parameter;
///
/// let p: Parameter = [Parameter::real(1.0), Parameter::real(2.0)]
///     .iter()
///     .collect();
/// assert!(matches!(p, Parameter::List(_)));
/// ```
///
/// serde data model
/// ----------------
///
/// This implements a [serde::Deserializer], i.e. a **data format**.
///
/// | Parameter   | serde data model |
/// |:------------|:-----------------|
/// | Integer     | i64              |
/// | Real        | f64              |
/// | String      | string           |
/// | Enumeration | unit_variant     |
/// | List        | seq              |
/// | NotProvided | unit             |
/// | Omitted     | unit             |
/// | Typed       | tuple_struct     |
/// | RValue      | map              |
///
/// See [the official document of serde data model](https://serde.rs/data-model.html) for detail.
///
/// - `Parameter::Typed` is mapped to "tuple_struct" e.g. `struct Rgb(u8, u8, u8)`
///   because the name of field members are not contained,
///   and thus "struct" like `struct S { r: u8, g: u8, b: u8 }` cannot be used.
/// - `Parameter::RValue` is mapped to "map"
///   i.e. an entity reference `#12` will be deserialized into `{ "Entity": 12 }`.
///
/// ### Examples
///
/// - For untyped parameters, e.g. real number, can be deserialized into any types
///   as far as compatible in terms of the serde data model.
///
/// ```
/// use serde::Deserialize;
/// use ruststep::ast::Parameter;
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
/// // Deserialize the `Parameter` sequence into `A` because serde allows upcasting seq to struct
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
/// - Typed parameter, e.g. `A(1)`
///   - FIXME: Type name check is not implemented yet.
///
/// ```
/// use serde::Deserialize;
/// use ruststep::parser::exchange;
/// use nom::Finish;
///
/// #[derive(Debug, Deserialize)]
/// struct A {
///     x: f64,
///     y: f64,
/// }
///
/// let (res, p) = exchange::parameter("A((1.0, 2.0))").finish().unwrap();
/// assert_eq!(res, "");
/// let a: A = Deserialize::deserialize(&p).unwrap();
/// dbg!(a);
/// ```
///
/// - For [RValue]
///
/// ```
/// use serde::Deserialize;
/// use ruststep::{parser::exchange, ast::RValue};
/// use nom::Finish;
///
/// let (res, p) = exchange::parameter("#11").finish().unwrap();
/// let a: RValue = Deserialize::deserialize(&p).unwrap();
/// assert_eq!(a, RValue::Entity(11))
/// ```
///
/// [serde::Deserializer]: https://docs.serde.rs/serde/trait.Deserializer.html
///
#[derive(Debug, Clone, PartialEq)]
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

    /// A reference to entity or value
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

impl<'de, 'param> de::Deserializer<'de> for &'param Parameter {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Parameter::Typed { name, ty } => visitor.visit_map(de::value::MapDeserializer::new(
                [(name.as_str(), &**ty)].iter().cloned(),
            )),
            Parameter::Integer(val) => visitor.visit_i64(*val),
            Parameter::Real(val) => visitor.visit_f64(*val),
            Parameter::String(val) => visitor.visit_str(val),
            Parameter::List(params) => {
                visitor.visit_seq(de::value::SeqDeserializer::new(params.iter()))
            }
            Parameter::RValue(rvalue) => de::Deserializer::deserialize_any(rvalue, visitor),
            Parameter::NotProvided | Parameter::Omitted => visitor.visit_unit(),
            Parameter::Enumeration(_) => unimplemented!(),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        struct tuple_struct map enum identifier ignored_any
    }
}

// To support SeqDeserializer
impl<'de, 'param> de::IntoDeserializer<'de, crate::error::Error> for &'param Parameter {
    type Deserializer = Self;
    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> Deserialize<'de> for Parameter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(ParameterVisitor {})
    }
}

struct ParameterVisitor {}

impl<'de> de::Visitor<'de> for ParameterVisitor {
    type Value = Parameter;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Parameter")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Parameter::integer(value))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Parameter::real(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Parameter::string(value))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut components: Vec<Parameter> = Vec::new();
        while let Some(component) = seq.next_element()? {
            components.push(component);
        }
        Ok(Parameter::List(components))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let (name, ty) = map.next_entry()?.unwrap();
        Ok(Parameter::Typed { name, ty })
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: de::EnumAccess<'de>,
    {
        let (kind, value): (String, _) = data.variant()?;
        match kind.as_str() {
            "Entity" => {
                let id = value.newtype_variant()?;
                return Ok(Parameter::RValue(RValue::Entity(id)));
            }
            _ => unimplemented!("enum to Parameter is not implemented yet"),
        }
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

    #[test]
    fn deserialize_identity() {
        let p = Parameter::integer(2);
        let q: Parameter = Deserialize::deserialize(&p).unwrap();
        assert_eq!(p, q);

        let p = Parameter::real(2.0);
        let q: Parameter = Deserialize::deserialize(&p).unwrap();
        assert_eq!(p, q);

        let p = Parameter::string("vim");
        let q: Parameter = Deserialize::deserialize(&p).unwrap();
        assert_eq!(p, q);

        let p: Parameter = [Parameter::integer(1), Parameter::real(2.0)]
            .iter()
            .collect();
        let q: Parameter = Deserialize::deserialize(&p).unwrap();
        assert_eq!(p, q);

        let (res, p) = crate::parser::exchange::parameter("B((1.0, A((2.0, 3.0))))")
            .finish()
            .unwrap();
        assert_eq!(res, "");
        let q: Parameter = Deserialize::deserialize(&p).unwrap();
        assert_eq!(p, q);
    }
}
