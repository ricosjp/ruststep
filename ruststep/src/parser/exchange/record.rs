use crate::parser::exchange::*;
use serde::{de, forward_to_deserialize_any};

/// A struct typed in EXPRESS schema
///
/// serde::Deserialize
/// -------------------
///
/// ```
/// use nom::Finish;
/// use serde::Deserialize;
/// use ruststep::parser::exchange;
///
/// #[derive(Debug, Deserialize)]
/// struct MyStruct {
///     x: f64,
///     y: f64,
/// }
///
/// let (_, record) = exchange::simple_record("MY_STRUCT(1.0, 2.0)").finish().unwrap();
/// let a: MyStruct = Deserialize::deserialize(&record).unwrap();
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub name: String,
    pub parameters: Vec<Parameter>,
}

impl<'de, 'record> de::Deserializer<'de> for &'record Record {
    type Error = crate::error::Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::invalid_type(
            de::Unexpected::Other("any"),
            &self.name.as_str(),
        ))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let seq = de::value::SeqDeserializer::new(self.parameters.iter());
        visitor.visit_seq(seq)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum identifier ignored_any
    }
}
