use serde::{de, forward_to_deserialize_any, Deserialize};

/// Left hand side value
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum LValue {
    /// Like `#11`
    Entity(u64),
    /// Like `@11`
    Value(u64),
}

/// Reference to a value in the exchange structure e.g. `#11`,
/// or value defined in EXPRESS schema e.g. `@CONST_VALUE`
///
/// serde::Deserialize
/// ------------------
/// [Deserialize] is derived without `#[serde(...)]` attribute,
/// which means it is "externally tagged" as described in [enum representations].
/// For example, `RValue::Entity(11)` will be deserialized from `{ "Entity": 11 }` in serde data model.
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
            RValue::Entity(id) => visitor.visit_map(de::value::MapDeserializer::new(
                [("Entity", *id)].iter().cloned(),
            )),
            RValue::Value(id) => visitor.visit_map(de::value::MapDeserializer::new(
                [("Value", *id)].iter().cloned(),
            )),
            RValue::ConstantEntity(name) => visitor.visit_map(de::value::MapDeserializer::new(
                [("ConstantEntity", name.clone())].iter().cloned(),
            )),
            RValue::ConstantValue(name) => visitor.visit_map(de::value::MapDeserializer::new(
                [("ConstantValue", name.clone())].iter().cloned(),
            )),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        enum tuple_struct struct map identifier ignored_any
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Deserializer (above) and Deserialize (auto-derived) implementations
    // must match to reproduce the original value
    #[test]
    fn deserialize_identity() {
        let value = RValue::Entity(11);
        let a: RValue = Deserialize::deserialize(&value).unwrap();
        assert_eq!(a, value);

        let value = RValue::Value(11);
        let a: RValue = Deserialize::deserialize(&value).unwrap();
        assert_eq!(a, value);

        let value = RValue::ConstantEntity("Const1".into());
        let a: RValue = Deserialize::deserialize(&value).unwrap();
        assert_eq!(a, value);

        let value = RValue::ConstantValue("Const1".into());
        let a: RValue = Deserialize::deserialize(&value).unwrap();
        assert_eq!(a, value);
    }
}
