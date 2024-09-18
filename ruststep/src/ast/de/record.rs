use crate::ast::*;
use serde::{
    de::{self, IntoDeserializer},
    forward_to_deserialize_any,
};

impl<'de, 'record> de::Deserializer<'de> for &'record Record {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(RecordDeserializer::new(&self.name, &self.parameter))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if name == self.name {
            if let Parameter::List(ref parameters) = self.parameter {
                return visitor.visit_map(RecordStructDeserializer::new(fields, parameters));
            }
        }
        Self::deserialize_any(self, visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum identifier ignored_any
    }
}

/// Deserializer corresponding to a single-key map like `{ "A": [1.0, 2.0] }`
#[derive(Debug)]
pub struct RecordDeserializer<'record> {
    key: Option<&'record str>,
    value: &'record Parameter,
}

impl<'de, 'record> de::Deserializer<'de> for RecordDeserializer<'record> {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
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

impl<'record> RecordDeserializer<'record> {
    pub fn new(key: &'record str, value: &'record Parameter) -> Self {
        RecordDeserializer {
            key: Some(key),
            value,
        }
    }
}

// Entry point of `visit_map`
impl<'de, 'record> de::MapAccess<'de> for RecordDeserializer<'record> {
    type Error = crate::error::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some(key) = self.key.take() {
            let key: de::value::StrDeserializer<Self::Error> = key.into_deserializer();
            let key: K::Value = seed.deserialize(key)?;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let value: V::Value = seed.deserialize(self.value)?;
        Ok(value)
    }
}

/// Deserialize [Record] as a struct
#[derive(Debug)]
pub struct RecordStructDeserializer<'a> {
    cursor: usize,
    fields: &'a [&'static str],
    parameters: &'a [Parameter],
}

impl<'a> RecordStructDeserializer<'a> {
    pub fn new(fields: &'a [&'static str], parameters: &'a [Parameter]) -> Self {
        Self {
            cursor: 0,
            fields,
            parameters,
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for RecordStructDeserializer<'a> {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
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

impl<'de, 'a> de::MapAccess<'de> for RecordStructDeserializer<'a> {
    type Error = crate::error::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.cursor < self.fields.len() {
            let key: de::value::StrDeserializer<Self::Error> =
                self.fields[self.cursor].into_deserializer();
            let key: K::Value = seed.deserialize(key)?;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        assert!(self.cursor < self.parameters.len());
        let value: V::Value = seed.deserialize(&self.parameters[self.cursor])?;
        self.cursor += 1;
        Ok(value)
    }
}
