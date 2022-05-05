use crate::ast::{de::RecordDeserializer, *};
use serde::{
    de::{self, IntoDeserializer},
    forward_to_deserialize_any,
};

impl<'de> de::IntoDeserializer<'de, crate::error::Error> for Name {
    type Deserializer = RecordDeserializer;
    fn into_deserializer(self) -> RecordDeserializer {
        match self {
            Name::Entity(id) => RecordDeserializer::new("Entity", Parameter::Integer(id as i64)),
            Name::Value(id) => RecordDeserializer::new("Value", Parameter::Integer(id as i64)),
            Name::ConstantEntity(name) => {
                RecordDeserializer::new("ConstantEntity", Parameter::String(name))
            }
            Name::ConstantValue(name) => {
                RecordDeserializer::new("ConstantValue", Parameter::String(name))
            }
        }
    }
}

impl<'de, 'value> de::Deserializer<'de> for &'value Name {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(self.clone().into_deserializer())
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        enum tuple_struct struct map identifier ignored_any
    }
}
