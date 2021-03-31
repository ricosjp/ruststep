use super::exchange::Parameter;
use crate::tables::*;
use serde::{
    de::{self, IntoDeserializer, VariantAccess},
    forward_to_deserialize_any, Deserialize,
};
use std::{fmt, marker::PhantomData};

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

/// Owned value or reference to entity/value
#[derive(Debug, Clone, PartialEq)]
pub enum PlaceHolder<T> {
    Ref(RValue),
    Owned(T),
}

impl<T: Holder> PlaceHolder<T> {
    /// Get owned value, or look up entity table and clone it for a reference.
    pub fn into_owned<Table>(self, table: &Table) -> Result<T::Owned, crate::error::Error>
    where
        T: Holder<Table = Table> + Clone,
        Table: EntityTable<T>,
    {
        let value = match self {
            PlaceHolder::Ref(id) => match id {
                RValue::Entity(id) => table.get_entity(id)?.clone(),
                _ => unimplemented!("ENTITY is only supported now"),
            },
            PlaceHolder::Owned(a) => a,
        };
        Ok(value.into_owned(table)?)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for PlaceHolder<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        // Dispatched dynamically:
        //
        // For Ref(RValue)
        // ----------------
        // PlaceHolder::deserialize(RValue)
        // > RValue::deserialize_struct(PlaceHolderVisitor)
        // > (forward_to_deserialize_any)
        // > RValue::deserialize_any(PlaceHolderVisitor)
        // > PlaceHolderVisitor::visit_enum(MapAccessDeserializer)
        //
        // For Owned(T)
        // -------------
        // PlaceHolder::deserialize(Record)
        // > Record::deserialize_struct(PlaceHolderVisitor)
        // > PlaceHolderVisitor::visit_seq(SeqDeserializer)
        deserializer.deserialize_struct(
            std::any::type_name::<T>(),
            &[],
            PlaceHolderVisitor::<T> {
                phantom: PhantomData,
            },
        )
    }
}

struct PlaceHolderVisitor<T> {
    phantom: PhantomData<T>,
}

impl<'de, T: Deserialize<'de>> de::Visitor<'de> for PlaceHolderVisitor<T> {
    type Value = PlaceHolder<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "PlaceHolder<{}>", std::any::type_name::<T>())
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(PlaceHolder::Owned(T::deserialize(v.into_deserializer())?))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(PlaceHolder::Owned(T::deserialize(v.into_deserializer())?))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(PlaceHolder::Owned(T::deserialize(v.into_deserializer())?))
    }

    // For Ref(RValue)
    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: de::EnumAccess<'de>,
    {
        let (key, variant): (String, _) = data.variant()?;
        match key.as_str() {
            "Entity" => {
                let value: u64 = variant.newtype_variant()?;
                Ok(PlaceHolder::Ref(RValue::Entity(value)))
            }
            "Value" => {
                let value: u64 = variant.newtype_variant()?;
                Ok(PlaceHolder::Ref(RValue::Value(value)))
            }
            "ConstantEntity" => {
                let name: String = variant.newtype_variant()?;
                Ok(PlaceHolder::Ref(RValue::ConstantEntity(name)))
            }
            "ConstantValue" => {
                let name: String = variant.newtype_variant()?;
                Ok(PlaceHolder::Ref(RValue::ConstantValue(name)))
            }
            _ => unreachable!("Invalid key while deserializing PlaceHolder"),
        }
    }

    // For Owned(T)
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut components: Vec<Parameter> = Vec::new();
        while let Some(component) = seq.next_element()? {
            components.push(component);
        }
        let seq = de::value::SeqDeserializer::new(components.iter());
        Ok(PlaceHolder::Owned(T::deserialize(seq).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::exchange;
    use nom::Finish;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct A {
        x: f64,
        y: f64,
    }

    #[test]
    fn place_holder() {
        let value = RValue::Entity(11);
        let a: PlaceHolder<A> = Deserialize::deserialize(&value).unwrap();
        dbg!(a);

        let value = RValue::ConstantValue("VIM".into());
        let a: PlaceHolder<A> = Deserialize::deserialize(&value).unwrap();
        dbg!(a);

        let (_, record) = exchange::simple_record("A(1.0, 2.0)").finish().unwrap();
        let a: PlaceHolder<A> = Deserialize::deserialize(&record).unwrap();
        dbg!(a);
    }
}
