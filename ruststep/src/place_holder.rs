use crate::{ast::*, tables::*};
use serde::{
    de::{self, IntoDeserializer, VariantAccess},
    Deserialize,
};
use std::{fmt, marker::PhantomData};

/// Owned value or reference through entity/value id
#[derive(Debug, Clone, PartialEq)]
pub enum PlaceHolder<T> {
    Ref(RValue),
    Owned(T),
}

impl<T: Holder> IntoOwned for PlaceHolder<T>
where
    T::Table: EntityTable<T>,
{
    type Owned = T::Owned;
    type Table = T::Table;
    /// Get owned value, or look up entity table and clone it for a reference.
    ///
    /// Errors
    /// -------
    /// - if table lookup failed, i.e. unknown entity id not registered in the table
    ///
    fn into_owned(self, table: &Self::Table) -> Result<T::Owned, crate::error::Error> {
        match self {
            PlaceHolder::Ref(id) => match id {
                RValue::Entity(id) => table.get_owned(id),
                _ => unimplemented!("ENTITY is only supported now"),
            },
            PlaceHolder::Owned(a) => a.into_owned(table),
        }
    }
}

impl<T: Holder> From<T> for PlaceHolder<T> {
    fn from(owned: T) -> Self {
        PlaceHolder::Owned(owned)
    }
}

impl<T> From<RValue> for PlaceHolder<T> {
    fn from(rvalue: RValue) -> Self {
        PlaceHolder::Ref(rvalue)
    }
}

impl<'de, T: Holder + WithVisitor + Deserialize<'de>> Deserialize<'de> for PlaceHolder<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple_struct(
            T::name(),
            T::attr_len(),
            PlaceHolderVisitor::<T>::default(),
        )
    }
}

pub struct PlaceHolderVisitor<T> {
    phantom: PhantomData<T>,
}

impl<T> Default for PlaceHolderVisitor<T> {
    fn default() -> Self {
        PlaceHolderVisitor {
            phantom: PhantomData,
        }
    }
}

impl<'de, T: Deserialize<'de> + WithVisitor> de::Visitor<'de> for PlaceHolderVisitor<T> {
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

    fn visit_seq<A>(self, seq: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let visitor = T::visitor_new();
        Ok(PlaceHolder::Owned(visitor.visit_seq(seq)?))
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

    // Entry point for Record or Parameter::Typed
    fn visit_map<A>(self, map: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let visitor = T::visitor_new();
        Ok(PlaceHolder::Owned(visitor.visit_map(map)?))
    }
}

impl<T> WithVisitor for PlaceHolder<T>
where
    T: WithVisitor + for<'de> de::Deserialize<'de>,
{
    type Visitor = PlaceHolderVisitor<T>;
    fn visitor_new() -> Self::Visitor {
        PlaceHolderVisitor {
            phantom: PhantomData,
        }
    }
}
