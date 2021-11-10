use crate::{ast::*, error::*, parser::exchange::simple_record};
use serde::{de, forward_to_deserialize_any};
use std::str::FromStr;

/// A struct typed in EXPRESS schema, e.g. `A(1.0, 2.0)`
///
/// ```
/// use ruststep::ast::{Record, Parameter};
/// use std::str::FromStr;
///
/// let record = Record::from_str("A(1, 2)").unwrap();
/// assert_eq!(
///     record,
///     Record {
///         name: "A".to_string(),
///         parameters: vec![Parameter::Integer(1), Parameter::Integer(2)]
///     }
/// )
/// ```
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
        visitor.visit_map(SingleMapDeserializer::new(
            &self.name,
            self.parameters.iter().collect::<Vec<&Parameter>>(),
        ))
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        struct tuple_struct map enum identifier ignored_any
    }
}

impl FromStr for Record {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        use nom::Finish;
        let input = input.trim();
        let (_input, record) = simple_record(input)
            .finish()
            .map_err(|err| TokenizeFailed::new(input, err))?;
        Ok(record)
    }
}
