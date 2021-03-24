use serde::{de, forward_to_deserialize_any, Deserialize};

#[cfg(doc)] // for doc-link
use super::exchange::Record;

/// Left hand side value
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum LValue {
    /// Like `#11`
    Entity(u64),
    /// Like `@11`
    Value(u64),
}

/// Right hand side value
///
/// serde::Deserializer
/// -------------------
///
/// Since [RValue] may appear in [Record], this should supports [serde::Deserializer]
/// as like done in [serde::de::value].
/// This enum is also [serde::Deserialize].
/// [Deserialize::deserialize] returns same value as following:
///
/// ```
/// use serde::Deserialize;
/// use ruststep::parser::value::RValue;
///
/// let value = RValue::Entity(11);
/// let a: RValue = Deserialize::deserialize(&value).unwrap();
/// assert_eq!(a, value);
///
/// let value = RValue::Value(11);
/// let a: RValue = Deserialize::deserialize(&value).unwrap();
/// assert_eq!(a, value);
///
/// let value = RValue::ConstantEntity("Const1".into());
/// let a: RValue = Deserialize::deserialize(&value).unwrap();
/// assert_eq!(a, value);
///
/// let value = RValue::ConstantValue("Const1".into());
/// let a: RValue = Deserialize::deserialize(&value).unwrap();
/// assert_eq!(a, value);
/// ```
///
/// Enum representation
/// --------------------
///
/// [Deserialize] is derived without `#[serde(...)]` attribute,
/// which means it is "externally tagged" as described in [enum representations].
/// For example, `RValue::Entity(11)` will be deserialized from `{ "Entity": 11 }`.
///
/// [enum representations]: https://serde.rs/enum-representations.html
///
#[derive(Debug, Clone, PartialEq, Deserialize)]
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
            RValue::Entity(id) => visitor.visit_enum(de::value::MapAccessDeserializer::new(
                de::value::MapDeserializer::new([("Entity", *id)].iter().cloned()),
            )),
            RValue::Value(id) => visitor.visit_enum(de::value::MapAccessDeserializer::new(
                de::value::MapDeserializer::new([("Value", *id)].iter().cloned()),
            )),
            RValue::ConstantEntity(name) => visitor.visit_enum(
                de::value::MapAccessDeserializer::new(de::value::MapDeserializer::new(
                    [("ConstantEntity", name.clone())].iter().cloned(),
                )),
            ),
            RValue::ConstantValue(name) => visitor.visit_enum(
                de::value::MapAccessDeserializer::new(de::value::MapDeserializer::new(
                    [("ConstantValue", name.clone())].iter().cloned(),
                )),
            ),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        enum tuple_struct struct map identifier ignored_any
    }
}
