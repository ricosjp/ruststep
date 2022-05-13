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
    value: Parameter,
}

impl<'de, 'record> de::IntoDeserializer<'de, crate::error::Error> for &'record Record {
    type Deserializer = RecordDeserializer;
    fn into_deserializer(self) -> RecordDeserializer {
        RecordDeserializer {
            key: Some(self.name.to_string()),
            value: *self.parameter.clone(),
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
            value: value,
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
        let value: V::Value = seed.deserialize(&self.value)?;
        Ok(value)
    }
}

// Note for understanding serde enum types
// ----------------------------------------
//
// In serde impl, we have to implement `VariantAccess` and `EnumAccess`
// since `Visitor::visit_enum` requires `EnumAccess` and it requires `VariantAccess`.
// `VariantAccess` and `EnumAccess` traits are used for 4 data models:
//
// - "unit_variant"    e.g. the `E::A` and `E::B` in `enum E { A, B }`
// - "newtype_variant" e.g. the `E::N` in `enum E { N(u8) }`
// - "tuple_variant"   e.g. the `E::T` in `enum E { T(u8, u8) }`
// - "struct_variant"  e.g. the `E::S` in `enum E { S { r: u8, g: u8, b: u8 } }`
//
// Roughly, `EnumAccess` determines which variant are used e.g. `E::N` in above "newtype_variant" case,
// and `VariantAccess` determines its component e.g. `1u8`.
// These are composed into `E::N(1u8)` in `Visitor::visit_enum`.
//
impl<'de, 'name> de::EnumAccess<'de> for &'name Name {
    type Error = crate::error::Error;
    type Variant = Self;
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let key: de::value::StrDeserializer<Self::Error> = match self {
            Name::Entity(_) => "Entity",
            Name::Value(_) => "Value",
            Name::ConstantEntity(_) => "ConstantEntity",
            Name::ConstantValue(_) => "ConstantValue",
        }
        .into_deserializer();
        let key: V::Value = seed.deserialize(key)?;
        Ok((key, self))
    }
}

impl<'de, 'name> de::VariantAccess<'de> for &'name Name {
    type Error = crate::error::Error;

    fn unit_variant(self) -> Result<()> {
        let unexp = de::Unexpected::NewtypeVariant;
        Err(de::Error::invalid_type(unexp, &"unit variant"))
    }

    fn newtype_variant_seed<D>(self, seed: D) -> Result<D::Value>
    where
        D: de::DeserializeSeed<'de>,
    {
        match self {
            Name::Entity(id) | Name::Value(id) => seed.deserialize(id.into_deserializer()),
            Name::ConstantEntity(name) | Name::ConstantValue(name) => {
                seed.deserialize(name.as_str().into_deserializer())
            }
        }
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let unexp = de::Unexpected::NewtypeVariant;
        Err(de::Error::invalid_type(unexp, &"tuple variant"))
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let unexp = de::Unexpected::NewtypeVariant;
        Err(de::Error::invalid_type(unexp, &"struct variant"))
    }
}
