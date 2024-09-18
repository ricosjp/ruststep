use crate::ast::*;
use serde::{
    de::{self, IntoDeserializer},
    forward_to_deserialize_any,
};

impl<'de, 'record> de::Deserializer<'de> for &'record SubSuperRecord {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(SubSuperRecordDeserializer::new(self.0.as_slice()))
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        struct tuple_struct map enum identifier ignored_any
    }
}

#[derive(Debug)]
pub struct SubSuperRecordDeserializer<'record> {
    keywords: Vec<&'record str>,
    parameters: Vec<&'record Parameter>,
}

impl<'record> SubSuperRecordDeserializer<'record> {
    pub fn new(records: &'record [Record]) -> Self {
        Self {
            keywords: records.iter().map(|x| x.name.as_str()).collect(),
            parameters: records.iter().map(|x| &x.parameter).collect(),
        }
    }
}

// Entry point of `visit_map`
impl<'de, 'record> de::MapAccess<'de> for SubSuperRecordDeserializer<'record> {
    type Error = crate::error::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some(key) = self.keywords.pop() {
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
        if let Some(value) = self.parameters.pop() {
            let value: V::Value = seed.deserialize(value)?;
            Ok(value)
        } else {
            unreachable!()
        }
    }
}
