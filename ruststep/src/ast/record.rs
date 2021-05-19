use crate::{ast::*, error::*};
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
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let seq = de::value::SeqDeserializer::new(self.parameters.iter());
        visitor.visit_seq(seq)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        struct tuple_struct map enum identifier ignored_any
    }
}
