use crate::{error::*, step::*};
use serde::{de, forward_to_deserialize_any, ser, Deserialize};
use std::{convert::TryFrom, fmt};

/// Primitive value type in STEP data
///
/// Parse
/// ------
///
/// ```
/// use nom::Finish;
/// use ruststep::{parser::exchange, step::Parameter};
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
/// use ruststep::{parser::exchange, step::Parameter};
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
/// use ruststep::step::Parameter;
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
/// use ruststep::step::Parameter;
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
/// use ruststep::{parser::exchange, step::RValue};
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
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
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
            Parameter::NotProvided | Parameter::Omitted => visitor.visit_none(),
            Parameter::Enumeration(_) => unimplemented!(),
        }
    }

    fn deserialize_struct<V>(
        self,
        _struct_name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
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
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
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

    fn visit_i64<E>(self, value: i64) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Parameter::integer(value))
    }

    fn visit_f64<E>(self, value: f64) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Parameter::real(value))
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Parameter::string(value))
    }

    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut components: Vec<Parameter> = Vec::new();
        while let Some(component) = seq.next_element()? {
            components.push(component);
        }
        Ok(Parameter::List(components))
    }

    fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let (name, ty) = map.next_entry()?.unwrap();
        Ok(Parameter::Typed { name, ty })
    }
}

impl<'param> ser::Serializer for &'param mut Parameter {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i64(self, v: i64) -> Result<()> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_i64(i64::try_from(v).expect("integer larger than i64::MAX is not supported"))
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }
    fn serialize_f64(self, v: f64) -> Result<()> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_str(self, v: &str) -> Result<()> {
        todo!()
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        todo!()
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<()> {
        todo!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

impl<'param> ser::SerializeSeq for &'param mut Parameter {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }
    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'param> ser::SerializeTuple for &'param mut Parameter {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'param> ser::SerializeTupleStruct for &'param mut Parameter {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'param> ser::SerializeTupleVariant for &'param mut Parameter {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'param> ser::SerializeMap for &'param mut Parameter {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'param> ser::SerializeStruct for &'param mut Parameter {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'param> ser::SerializeStructVariant for &'param mut Parameter {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
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
        let res: Result<u32> = Deserialize::deserialize(&p);
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
