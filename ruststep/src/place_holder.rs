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

impl<T: Holder> PlaceHolder<T> {
    /// Get owned value, or look up entity table and clone it for a reference.
    ///
    /// Errors
    /// -------
    /// - if table lookup failed, i.e. unknown entity id not registered in the table
    ///
    pub fn into_owned<Table>(self, table: &Table) -> Result<T::Owned, crate::error::Error>
    where
        T: Holder<Table = Table> + Clone,
        Table: EntityTable<T>,
    {
        match self {
            PlaceHolder::Ref(id) => match id {
                RValue::Entity(id) => table.get_owned(id),
                _ => unimplemented!("ENTITY is only supported now"),
            },
            PlaceHolder::Owned(a) => a.into_owned(table),
        }
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
        // > PlaceHolderVisitor::visit_enum(SingleMapDeserializer)
        //
        // For Owned(T)
        // -------------
        // PlaceHolder::deserialize(Record)
        // > (forward_to_deserialize_any)
        // > Record::deserialize_any(PlaceHolderVisitor)
        // > PlaceHolderVisitor::visit_seq(SeqDeserializer)
        deserializer.deserialize_struct(
            std::any::type_name::<T>(),
            &[],
            PlaceHolderVisitor::<T>::default(),
        )
    }
}

struct PlaceHolderVisitor<T> {
    phantom: PhantomData<T>,
}

impl<T> Default for PlaceHolderVisitor<T> {
    fn default() -> Self {
        PlaceHolderVisitor {
            phantom: PhantomData,
        }
    }
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
