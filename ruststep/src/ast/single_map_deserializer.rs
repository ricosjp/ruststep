use serde::de::{self, IntoDeserializer};

/// Deserializer corresponding to a single-key map like `{ "A": [1.0, 2.0] }`
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
