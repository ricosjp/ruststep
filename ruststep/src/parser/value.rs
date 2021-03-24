use serde::{de, forward_to_deserialize_any, Deserialize};

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
/// Since RValue may appear in [Record], this should supports [serde::Deserializer]
/// as like done in [serde::de::value].
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
/// [serde::de::value]: https://docs.serde.rs/serde/de/value/index.html
/// [serde::Deserializer]: https://docs.serde.rs/serde/trait.Deserializer.html
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

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum identifier ignored_any
    }
}
