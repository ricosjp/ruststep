use crate::{ast::*, error::*};
use serde::de::{self, IntoDeserializer};

// Note for understanding serde enum types
// ----------------------------------------
//
// In serde impl, we have to implement `VariantAccess` and `EnumAccess`
// since `Visitor::visit_enum` requires `EnumAccess` and it requires `VariantAccess`.
// `VariantAccess` and `EnumAccess` traits are used for 4 data models:
//
// - "unit_variant"    e.g. the `E::A` and `E::B` in `enum E { A, B }`
// - "newtype_variant" e.g. the `E::N` in `enum E { N(u8) }`
// - "tuple_variant"   e.g. the `E::T` in `enum E { T(u8, u8) }`
// - "struct_variant"  e.g. the `E::S` in `enum E { S { r: u8, g: u8, b: u8 } }`
//
// Roughly, `EnumAccess` determines which variant are used e.g. `E::N` in above "newtype_variant" case,
// and `VariantAccess` determines its component e.g. `1u8`.
// These are composed into `E::N(1u8)` in `Visitor::visit_enum`.
//
impl<'de, 'name> de::EnumAccess<'de> for &'name Name {
    type Error = crate::error::Error;
    type Variant = Self;
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let key: de::value::StrDeserializer<Self::Error> = match self {
            Name::Entity(_) => "Entity",
            Name::Value(_) => "Value",
            Name::ConstantEntity(_) => "ConstantEntity",
            Name::ConstantValue(_) => "ConstantValue",
        }
        .into_deserializer();
        let key: V::Value = seed.deserialize(key)?;
        Ok((key, self))
    }
}

impl<'de, 'name> de::VariantAccess<'de> for &'name Name {
    type Error = crate::error::Error;

    fn unit_variant(self) -> Result<()> {
        let unexp = de::Unexpected::NewtypeVariant;
        Err(de::Error::invalid_type(unexp, &"unit variant"))
    }

    fn newtype_variant_seed<D>(self, seed: D) -> Result<D::Value>
    where
        D: de::DeserializeSeed<'de>,
    {
        match self {
            Name::Entity(id) | Name::Value(id) => seed.deserialize(id.into_deserializer()),
            Name::ConstantEntity(name) | Name::ConstantValue(name) => {
                seed.deserialize(name.as_str().into_deserializer())
            }
        }
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let unexp = de::Unexpected::NewtypeVariant;
        Err(de::Error::invalid_type(unexp, &"tuple variant"))
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let unexp = de::Unexpected::NewtypeVariant;
        Err(de::Error::invalid_type(unexp, &"struct variant"))
    }
}
