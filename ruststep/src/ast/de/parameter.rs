use super::*;
use crate::ast::*;
use inflector::Inflector;
use serde::{
    de::{self, IntoDeserializer},
    forward_to_deserialize_any,
};

impl<'de, 'param> de::Deserializer<'de> for &'param Parameter {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Parameter::Typed { keyword, parameter } => {
                visitor.visit_map(RecordDeserializer::new(keyword, parameter))
            }
            Parameter::Integer(val) => visitor.visit_i64(*val),
            Parameter::Real(val) => visitor.visit_f64(*val),
            Parameter::String(val) => visitor.visit_str(val),
            Parameter::List(params) => visitor.visit_seq(SeqDeserializer::new(params)),
            Parameter::Ref(name) => visitor.visit_enum(name),
            Parameter::NotProvided | Parameter::Omitted => visitor.visit_none(),
            Parameter::Enumeration(variant) => {
                visitor.visit_enum(variant.to_class_case().into_deserializer())
            }
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if let Parameter::Enumeration(variant) = self {
            match variant.as_str() {
                "T" => visitor.visit_bool(true),
                "TRUE" => visitor.visit_bool(true),
                "F" => visitor.visit_bool(false),
                "FALSE" => visitor.visit_bool(false),
                _ => visitor.visit_enum(variant.to_class_case().into_deserializer()),
            }
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if matches!(self, Parameter::NotProvided | Parameter::Omitted) {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct newtype_struct seq tuple
        struct tuple_struct map enum identifier ignored_any
    }
}

#[derive(Debug)]
pub struct SeqDeserializer<'p> {
    cursor: usize,
    parameters: &'p [Parameter],
}

impl<'p> SeqDeserializer<'p> {
    pub fn new(parameters: &'p [Parameter]) -> Self {
        SeqDeserializer {
            cursor: 0,
            parameters,
        }
    }
}

impl<'de, 'p> de::Deserializer<'de> for SeqDeserializer<'p> {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        struct tuple_struct map enum identifier ignored_any
    }
}

impl<'de, 'p> de::SeqAccess<'de> for SeqDeserializer<'p> {
    type Error = crate::error::Error;

    fn size_hint(&self) -> Option<usize> {
        Some(self.parameters.len())
    }

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.cursor < self.parameters.len() {
            let value = seed.deserialize(&self.parameters[self.cursor])?;
            self.cursor += 1;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}
