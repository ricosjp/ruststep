use crate::{ast::*, error::*, parser::exchange::simple_record};
use serde::{
    de::{self, IntoDeserializer},
    forward_to_deserialize_any,
};
use std::str::FromStr;

/// A struct typed in EXPRESS schema, e.g. `A(1.0, 2.0)`
///
/// ```
/// use ruststep::ast::{Record, Parameter};
/// use std::str::FromStr;
///
/// let record = Record::from_str("A(1, 2)").unwrap();
/// assert_eq!(
///     record,
///     Record {
///         name: "A".to_string(),
///         parameter: Box::new(vec![Parameter::Integer(1), Parameter::Integer(2)].into())
///     }
/// )
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub name: String,
    pub parameter: Box<Parameter>,
}

impl<'de, 'record> de::Deserializer<'de> for &'record Record {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(self.into_deserializer())
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        struct tuple_struct map enum identifier ignored_any
    }
}

impl FromStr for Record {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        use nom::Finish;
        let input = input.trim();
        let (_input, record) = simple_record(input)
            .finish()
            .map_err(|err| TokenizeFailed::new(input, err))?;
        Ok(record)
    }
}

/// Deserializer corresponding to a single-key map like `{ "A": [1.0, 2.0] }`
#[derive(Debug)]
pub struct RecordDeserializer {
    key: Option<String>,
    value: Option<Parameter>,
}

impl<'de, 'record> de::IntoDeserializer<'de, crate::error::Error> for &'record Record {
    type Deserializer = RecordDeserializer;
    fn into_deserializer(self) -> RecordDeserializer {
        RecordDeserializer {
            key: Some(self.name.to_string()),
            value: Some(*self.parameter.clone()),
        }
    }
}

impl<'de> de::Deserializer<'de> for RecordDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        struct tuple_struct map enum identifier ignored_any
    }
}

impl RecordDeserializer {
    pub fn new(key: &str, value: Parameter) -> Self {
        RecordDeserializer {
            key: Some(key.to_string()),
            value: Some(value),
        }
    }
}

// Entry point of `visit_map`
impl<'de> de::MapAccess<'de> for RecordDeserializer {
    type Error = crate::error::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some(key) = self.key.take() {
            let key: de::value::StrDeserializer<Self::Error> = key.as_str().into_deserializer();
            let key: K::Value = seed.deserialize(key)?;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        if let Some(value) = self.value.take() {
            let value: V::Value = seed.deserialize(&value)?;
            Ok(value)
        } else {
            unreachable!("next_value_seed before next_key_seed is incorrect.")
        }
    }
}

// Entry point of `visit_enum`
impl<'de> de::EnumAccess<'de> for RecordDeserializer {
    type Error = crate::error::Error;
    type Variant = Self; // this requires `VariantAccess` (see below impl)

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        match de::MapAccess::next_key_seed(&mut self, seed)? {
            Some(key) => Ok((key, self)),
            None => Err(de::Error::invalid_type(de::Unexpected::Map, &"enum")),
        }
    }
}

// As serde document says,
//
// https://docs.serde.rs/serde/de/trait.VariantAccess.html
//
// > VariantAccess is a visitor that is created by the Deserializer
// > and passed to the Deserialize to deserialize the content of a particular enum variant.
//
// this trait is used for 4 data models:
//
// - "unit_variant"    e.g. the `E::A` and `E::B` in `enum E { A, B }`
// - "newtype_variant" e.g. the `E::N` in `enum E { N(u8) }`
// - "tuple_variant"   e.g. the `E::T` in `enum E { T(u8, u8) }`
// - "struct_variant"  e.g. the `E::S` in `enum E { S { r: u8, g: u8, b: u8 } }`
//
// But, `RecordDeserializer` is only used for "newtype_variant" case,
// and returns `Err` in other cases.
//
impl<'de> de::VariantAccess<'de> for RecordDeserializer {
    type Error = crate::error::Error;

    fn unit_variant(self) -> Result<()> {
        let unexp = de::Unexpected::Map;
        Err(de::Error::invalid_type(unexp, &"unit variant"))
    }

    fn newtype_variant_seed<D>(mut self, seed: D) -> Result<D::Value>
    where
        D: de::DeserializeSeed<'de>,
    {
        de::MapAccess::next_value_seed(&mut self, seed)
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let unexp = de::Unexpected::Map;
        Err(de::Error::invalid_type(unexp, &"tuple variant"))
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let unexp = de::Unexpected::Map;
        Err(de::Error::invalid_type(unexp, &"struct variant"))
    }
}
