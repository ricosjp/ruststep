use crate::parser::exchange::*;
use inflector::Inflector;
use serde::{de, forward_to_deserialize_any};

/// A struct typed in EXPRESS schema
///
/// serde::Deserialize
/// -------------------
///
/// Different from [Parameter], this checks the target struct name:
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
/// // `MyStruct` as Rust struct must be parsed from `MY_STRUCT` STEP record
/// let (_, record) = exchange::simple_record("MY_STRUCT(1.0, 2.0)").finish().unwrap();
/// let a: MyStruct = Deserialize::deserialize(&record).unwrap();
///
/// // Other type `YOUR_STRUCT` cannot be deserialized
/// // even if internal data `(f64, f64)` is matched.
/// let (_, record) = exchange::simple_record("YOUR_STRUCT(1.0, 2.0)").finish().unwrap();
/// let a: Result<MyStruct, _> = Deserialize::deserialize(&record);
/// assert!(a.is_err());
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
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if name != self.name.to_pascal_case() {
            return Err(de::Error::invalid_type(
                de::Unexpected::StructVariant,
                &self.name.as_str(),
            ));
        }
        let seq = de::value::SeqDeserializer::new(self.parameters.iter());
        visitor.visit_seq(seq)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum identifier ignored_any
    }
}

#[cfg(test)]
mod tests {
    use nom::Finish;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct MyStruct {
        x: f64,
        y: f64,
    }

    #[test]
    fn deserialize_record_to_struct() {
        let (res, record) = super::simple_record("MY_STRUCT(1.0, 2.0)")
            .finish()
            .unwrap();
        assert_eq!(res, "");
        let a: MyStruct = Deserialize::deserialize(&record).unwrap();
        dbg!(a);

        let (res, record) = super::simple_record("YOUR_STRUCT(1.0, 2.0)")
            .finish()
            .unwrap();
        assert_eq!(res, "");
        let a: Result<MyStruct, _> = Deserialize::deserialize(&record);
        assert!(a.is_err());
    }
}
