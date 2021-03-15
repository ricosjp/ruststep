//! Mapping STEP data into [serde data model]
//!
//! [serde data model]: https://serde.rs/data-model.html

use serde::{de, forward_to_deserialize_any, Deserialize};
use std::fmt;

use crate::parser::{Parameter, Record, UntypedParameter};

#[derive(Debug)]
pub struct RecordDeserializer<'de> {
    record: &'de Record,
    // Position of parameter to be used
    position: usize,
}

/// Interpret STEP record into a `Deserialize`-able structure
pub fn from_step_record<'de, T>(record: &'de Record) -> Result<T, DeserializeError>
where
    T: Deserialize<'de>,
{
    let mut de = RecordDeserializer {
        record,
        position: 0,
    };
    let t = T::deserialize(&mut de)?;
    Ok(t)
}

#[derive(Debug, thiserror::Error)]
pub enum DeserializeError {
    #[error("{0}")]
    Message(String),

    #[error("Reach to the end of record")]
    Eof,
}

impl de::Error for DeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::Message(msg.to_string())
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut RecordDeserializer<'de> {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.position >= self.record.parameters.len() {
            return Err(DeserializeError::Eof);
        }
        let value = match &self.record.parameters[self.position] {
            Parameter::Typed { name: _, ty: _ } => unimplemented!(),
            Parameter::Untyped(p) => match p {
                UntypedParameter::Integer(val) => visitor.visit_i64(*val)?,
                UntypedParameter::Real(val) => visitor.visit_f64(*val)?,
                UntypedParameter::String(val) => visitor.visit_str(val)?,
                _ => todo!(),
            },
            Parameter::Omitted => unimplemented!(),
        };
        self.position += 1;
        Ok(value)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

#[cfg(test)]
mod tests {
    use crate::parser;
    use nom::Finish;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct A {
        x: f64,
        y: f64,
    }

    #[test]
    fn test_from_step_record() {
        let (_, record) = parser::exchange::simple_record("A(1.0, 2.0)")
            .finish()
            .unwrap();
        let a: A = super::from_step_record(&record).unwrap();
        dbg!(a);
    }
}
