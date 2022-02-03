use super::{parameter::Parameter, SingleMapDeserializer};
use serde::{de, forward_to_deserialize_any, Deserialize};

#[cfg(doc)] // for doc-link
use super::Record;

/// Left hand side value
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum LValue {
    /// Like `#11`
    Entity(u64),
    /// Like `@11`
    Value(u64),
}

/// Right hand side value
#[derive(Debug, Clone, PartialEq)]
pub enum RValue {
    /// Like `#11`
    Entity(u64),
    /// Like `@11`
    Value(u64),
    /// Like `#CONST_ENTITY`
    ConstantEntity(String),
    /// Like `@CONST_VALUE`
    ConstantValue(String),
}

impl<'de, 'value> de::Deserializer<'de> for &'value RValue {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            RValue::Entity(id) => visitor.visit_enum(SingleMapDeserializer::new(
                "Entity",
                Parameter::Integer(*id as i64),
            )),
            RValue::Value(id) => visitor.visit_enum(SingleMapDeserializer::new(
                "Value",
                Parameter::Integer(*id as i64),
            )),
            RValue::ConstantEntity(name) => visitor.visit_enum(SingleMapDeserializer::new(
                "ConstantEntity",
                Parameter::String(name.clone()),
            )),
            RValue::ConstantValue(name) => visitor.visit_enum(SingleMapDeserializer::new(
                "ConstantValue",
                Parameter::String(name.clone()),
            )),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        enum tuple_struct struct map identifier ignored_any
    }
}
