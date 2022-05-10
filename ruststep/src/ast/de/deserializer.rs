use crate::{ast::*, error::*};
use serde::{
    de::{self, IntoDeserializer},
    forward_to_deserialize_any,
};

#[derive(Debug)]
pub struct SeqDeserializer {
    parameters: Vec<Parameter>,
}

impl SeqDeserializer {
    pub fn new(parameters: &[Parameter]) -> Self {
        SeqDeserializer {
            parameters: parameters.iter().rev().cloned().collect(),
        }
    }
}

impl<'de> de::SeqAccess<'de> for SeqDeserializer {
    type Error = Error;

    fn size_hint(&self) -> Option<usize> {
        Some(self.parameters.len())
    }

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if let Some(last) = self.parameters.pop() {
            let value = seed.deserialize(&last)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
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
