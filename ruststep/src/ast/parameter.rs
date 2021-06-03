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
/// assert_eq!(p, Parameter::from_iter(&[Parameter::string("ruststep"), Parameter::real(1.0)]));
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
///     ty: Box::new(Parameter::from_iter(&[Parameter::real(2.0), Parameter::real(3.0)])),
/// };
///
/// // B((1.0, a))
/// let b = Parameter::Typed {
///     name: "B".to_string(),
///     ty: Box::new(Parameter::from_iter(&[Parameter::real(1.0), a])),
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

    /// The special token dollar sign (`$`) is used to represent
    /// an object whose value is not provided in the exchange structure.
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

    pub fn from_iter<'a>(iter: impl IntoIterator<Item = &'a Parameter>) -> Self {
        std::iter::FromIterator::from_iter(iter)
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
            Parameter::Typed { name, ty } => {
                visitor.visit_map(SingleMapDeserializer::new(name, ty.as_ref()))
            }
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
        _struct_name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if let Parameter::Typed { name: _, ty } = self {
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

        let p = Parameter::from_iter(&[Parameter::integer(1), Parameter::real(2.0)]);
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
