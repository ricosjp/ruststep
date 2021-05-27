use serde::de::{self, IntoDeserializer};

/// Deserializer corresponding to a single-key map like `{ "A": [1.0, 2.0] }`
#[derive(Debug)]
pub struct SingleMapDeserializer<T> {
    key: Option<String>,
    value: Option<T>,
}

impl<T> SingleMapDeserializer<T> {
    pub fn new(key: &str, value: T) -> Self {
        SingleMapDeserializer {
            key: Some(key.to_string()),
            value: Some(value),
        }
    }
}

// Entry point of `visit_map`
impl<'de, T> de::MapAccess<'de> for SingleMapDeserializer<T>
where
    T: IntoDeserializer<'de, crate::error::Error>,
{
    type Error = crate::error::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some(key) = self.key.take() {
            let key: de::value::StrDeserializer<Self::Error> = key.as_str().into_deserializer();
            let key: K::Value = seed.deserialize(key)?;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        if let Some(value) = self.value.take() {
            let value = value.into_deserializer();
            let value: V::Value = seed.deserialize(value)?;
            Ok(value)
        } else {
            unreachable!("next_value_seed before next_key_seed is incorrect.")
        }
    }
}

// Entry point of `visit_enum`
impl<'de, T> de::EnumAccess<'de> for SingleMapDeserializer<T>
where
    T: IntoDeserializer<'de, crate::error::Error>,
{
    type Error = crate::error::Error;
    type Variant = Self; // this requires `VariantAccess` (see below impl)

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        match de::MapAccess::next_key_seed(&mut self, seed)? {
            Some(key) => Ok((key, self)),
            None => Err(de::Error::invalid_type(de::Unexpected::Map, &"enum")),
        }
    }
}

// As serde document says,
//
// https://docs.serde.rs/serde/de/trait.VariantAccess.html
//
// > VariantAccess is a visitor that is created by the Deserializer
// > and passed to the Deserialize to deserialize the content of a particular enum variant.
//
// this trait is used for 4 data models:
//
// - "unit_variant"    e.g. the `E::A` and `E::B` in `enum E { A, B }`
// - "newtype_variant" e.g. the `E::N` in `enum E { N(u8) }`
// - "tuple_variant"   e.g. the `E::T` in `enum E { T(u8, u8) }`
// - "struct_variant"  e.g. the `E::S` in `enum E { S { r: u8, g: u8, b: u8 } }`
//
// But, `SingleMapDeserializer` is only used for "newtype_variant" case,
// and returns `Err` in other cases.
//
impl<'de, T> de::VariantAccess<'de> for SingleMapDeserializer<T>
where
    T: IntoDeserializer<'de, crate::error::Error>,
{
    type Error = crate::error::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        let unexp = de::Unexpected::Map;
        Err(de::Error::invalid_type(unexp, &"unit variant"))
    }

    fn newtype_variant_seed<D>(mut self, seed: D) -> Result<D::Value, Self::Error>
    where
        D: de::DeserializeSeed<'de>,
    {
        de::MapAccess::next_value_seed(&mut self, seed)
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let unexp = de::Unexpected::Map;
        Err(de::Error::invalid_type(unexp, &"tuple variant"))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let unexp = de::Unexpected::Map;
        Err(de::Error::invalid_type(unexp, &"struct variant"))
    }
}
