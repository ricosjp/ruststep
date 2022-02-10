use super::{parameter::Parameter, RecordDeserializer};
use serde::{
    de::{self, IntoDeserializer},
    forward_to_deserialize_any, Deserialize,
};

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

impl<'de> de::IntoDeserializer<'de, crate::error::Error> for RValue {
    type Deserializer = RecordDeserializer;
    fn into_deserializer(self) -> RecordDeserializer {
        match self {
            RValue::Entity(id) => RecordDeserializer::new("Entity", Parameter::Integer(id as i64)),
            RValue::Value(id) => RecordDeserializer::new("Value", Parameter::Integer(id as i64)),
            RValue::ConstantEntity(name) => {
                RecordDeserializer::new("ConstantEntity", Parameter::String(name))
            }
            RValue::ConstantValue(name) => {
                RecordDeserializer::new("ConstantValue", Parameter::String(name))
            }
        }
    }
}

impl<'de, 'value> de::Deserializer<'de> for &'value RValue {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        dbg!(std::any::type_name::<V>());
        visitor.visit_enum(self.clone().into_deserializer())
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        enum tuple_struct struct map identifier ignored_any
    }
}
