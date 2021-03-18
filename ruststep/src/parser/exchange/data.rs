use crate::parser::{combinator::*, exchange::*, token::*};
use inflector::Inflector;
use nom::{branch::alt, Parser};
use serde::{de, forward_to_deserialize_any};

#[derive(Debug, Clone, PartialEq)]
pub struct DataSection {
    pub meta: Vec<Parameter>,
    pub entities: Vec<EntityInstance>,
}

/// data_section = `DATA` \[ `(` [parameter_list] `)` \] `;` [entity_instance_list] `ENDSEC;` .
pub fn data_section(input: &str) -> ParseResult<DataSection> {
    tuple_((
        tag_("DATA"),
        opt_(tuple_((char_('('), parameter_list, char_(')')))),
        char_(';'),
        entity_instance_list,
        tag_("ENDSEC;"),
    ))
    .map(|(_start, meta, _semicolon, entities, _end)| DataSection {
        meta: meta
            .map(|(_open, params, _close)| params)
            .unwrap_or_default(),
        entities,
    })
    .parse(input)
}

/// entity_instance_list = { [entity_instance] } .
pub fn entity_instance_list(input: &str) -> ParseResult<Vec<EntityInstance>> {
    many0_(entity_instance).parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityInstance {
    Simple { name: u64, record: Record },
    Complex { name: u64, subsuper: Vec<Record> },
}

/// entity_instance = [simple_entity_instance] | [complex_entity_instance] .
pub fn entity_instance(input: &str) -> ParseResult<EntityInstance> {
    alt((simple_entity_instance, complex_entity_instance)).parse(input)
}

/// simple_entity_instance = [entity_instance_name] `=` [simple_record] `;` .
pub fn simple_entity_instance(input: &str) -> ParseResult<EntityInstance> {
    tuple_((entity_instance_name, char_('='), simple_record, char_(';')))
        .map(|(name, _eq, record, _semicolon)| EntityInstance::Simple { name, record })
        .parse(input)
}

/// complex_entity_instance = [entity_instance_name] `=` [subsuper_record] `;` .
pub fn complex_entity_instance(input: &str) -> ParseResult<EntityInstance> {
    tuple_((
        entity_instance_name,
        char_('='),
        subsuper_record,
        char_(';'),
    ))
    .map(|(name, _eq, subsuper, _semicolon)| EntityInstance::Complex { name, subsuper })
    .parse(input)
}

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
///
/// #[derive(Debug, Deserialize)]
/// struct MyStruct {
///     x: f64,
///     y: f64,
/// }
///
/// // `MyStruct` as Rust struct must be parsed from `MY_STRUCT` STEP record
/// let (_, record) = super::simple_record("MY_STRUCT(1.0, 2.0)").finish().unwrap();
/// let a: MyStruct = Deserialize::deserialize(&record).unwrap();
///
/// // Other type `YOUR_STRUCT` cannot be deserialized
/// // even if internal data `(f64, f64)` is matched.
/// let (_, record) = super::simple_record("YOUR_STRUCT(1.0, 2.0)").finish().unwrap();
/// let a: Result<MyStruct, _> = Deserialize::deserialize(&record);
/// assert!(a.is_err());
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub name: String,
    pub parameters: Vec<Parameter>,
}

/// simple_record = [keyword] `(` \[ [parameter_list] \] `)` .
pub fn simple_record(input: &str) -> ParseResult<Record> {
    tuple_((keyword, char_('('), opt_(parameter_list), char_(')')))
        .map(|(name, _open, parameters, _close)| Record {
            name,
            parameters: parameters.unwrap_or_default(),
        })
        .parse(input)
}

/// simple_record_list = [simple_record] { [simple_record] } .
pub fn simple_record_list(input: &str) -> ParseResult<Vec<Record>> {
    many0_(simple_record).parse(input)
}

/// subsuper_record = `(` [simple_record_list] `)` .
pub fn subsuper_record(input: &str) -> ParseResult<Vec<Record>> {
    tuple_((char_('('), simple_record_list, char_(')')))
        .map(|(_open, records, _close)| records)
        .parse(input)
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

    #[test]
    fn simple_recode1() {
        let (res, record) = super::simple_record("A(1, 2.0)").finish().unwrap();
        dbg!(record);
        assert_eq!(res, "");
    }

    #[test]
    fn simple_recode2() {
        let (res, record) = super::simple_record(
            "LENGTH_MEASURE_WITH_UNIT( LENGTH_MEASURE( 1.00000000000000 ), #359 )",
        )
        .finish()
        .unwrap();
        dbg!(record);
        assert_eq!(res, "");
    }

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
