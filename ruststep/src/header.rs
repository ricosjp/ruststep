use crate::{ast::*, error::Result};
use serde::{de, Deserialize};

#[derive(Debug, Clone, PartialEq)]
pub struct FileDescription {
    description: Vec<String>,
    implementation_level: String,
}

impl<'de> de::Deserialize<'de> for FileDescription {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple_struct("FILE_DESCRIPTION", 2, FileDescriptionVisitor {})
    }
}

struct FileDescriptionVisitor;

impl<'de> ::serde::de::Visitor<'de> for FileDescriptionVisitor {
    type Value = FileDescription;
    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "FILE_DESCRIPTION")
    }

    fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: ::serde::de::SeqAccess<'de>,
    {
        if let Some(size) = seq.size_hint() {
            if size != 2 {
                todo!("Create another error and send it")
            }
        }
        let description = seq.next_element()?.unwrap();
        let implementation_level = seq.next_element()?.unwrap();
        Ok(FileDescription {
            description,
            implementation_level,
        })
    }

    // Entry point for Record or Parameter::Typed
    fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: ::serde::de::MapAccess<'de>,
    {
        let key: String = map
            .next_key()?
            .expect("Empty map cannot be accepted as ruststep Holder"); // this must be a bug, not runtime error
        if key != "FILE_DESCRIPTION" {
            todo!("Create Error type and send it")
        }
        let value = map.next_value()?; // send to Self::visit_seq
        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileName {
    name: String,
    time_stamp: String,
    author: Vec<String>,
    organization: Vec<String>,
    preprocessor_version: String,
    originating_system: String,
    authorization: String,
}

impl<'de> de::Deserialize<'de> for FileName {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple_struct("FILE_NAME", 7, FileNameVisitor {})
    }
}

struct FileNameVisitor;

impl<'de> ::serde::de::Visitor<'de> for FileNameVisitor {
    type Value = FileName;
    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "FILE_NAME")
    }

    fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: ::serde::de::SeqAccess<'de>,
    {
        if let Some(size) = seq.size_hint() {
            if size != 7 {
                todo!("Create another error and send it")
            }
        }
        let name = seq.next_element()?.unwrap();
        let time_stamp = seq.next_element()?.unwrap();
        let author = seq.next_element()?.unwrap();
        let organization = seq.next_element()?.unwrap();
        let preprocessor_version = seq.next_element()?.unwrap();
        let originating_system = seq.next_element()?.unwrap();
        let authorization = seq.next_element()?.unwrap();
        Ok(FileName {
            name,
            time_stamp,
            author,
            organization,
            preprocessor_version,
            originating_system,
            authorization,
        })
    }

    // Entry point for Record or Parameter::Typed
    fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: ::serde::de::MapAccess<'de>,
    {
        let key: String = map
            .next_key()?
            .expect("Empty map cannot be accepted as ruststep Holder"); // this must be a bug, not runtime error
        if key != "FILE_NAME" {
            todo!("Create Error type and send it")
        }
        let value = map.next_value()?; // send to Self::visit_seq
        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileSchema {
    schema: Vec<String>,
}

impl<'de> de::Deserialize<'de> for FileSchema {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple_struct("FILE_SCHEMA", 1, FileSchemaVisitor {})
    }
}

struct FileSchemaVisitor;

impl<'de> ::serde::de::Visitor<'de> for FileSchemaVisitor {
    type Value = FileSchema;
    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(formatter, "FILE_SCHEMA")
    }

    fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: ::serde::de::SeqAccess<'de>,
    {
        if let Some(size) = seq.size_hint() {
            if size != 1 {
                todo!("Create another error and send it")
            }
        }
        let schema = seq.next_element()?.unwrap();
        Ok(FileSchema { schema })
    }

    // Entry point for Record or Parameter::Typed
    fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: ::serde::de::MapAccess<'de>,
    {
        let key: String = map
            .next_key()?
            .expect("Empty map cannot be accepted as ruststep Holder"); // this must be a bug, not runtime error
        if key != "FILE_SCHEMA" {
            todo!("Create Error type and send it")
        }
        let value = map.next_value()?; // send to Self::visit_seq
        Ok(value)
    }
}

/// STEP-file HEADER section
///
/// There is a schema for HEADER section,
/// but we do not generate this structure from it to simplify build process.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    file_description: FileDescription,
    file_name: FileName,
    file_schema: FileSchema,
}

impl Header {
    pub fn from_records(records: &[Record]) -> Result<Self> {
        assert!(records.len() >= 3);
        let file_description = FileDescription::deserialize(&records[0])?;
        let file_name = FileName::deserialize(&records[1])?;
        let file_schema = FileSchema::deserialize(&records[2])?;
        Ok(Header {
            file_description,
            file_name,
            file_schema,
        })
    }
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn header() {
        // From ABC dataset example
        let header = r#"
        HEADER;
            FILE_DESCRIPTION( ( '' ), ' ' );
            FILE_NAME( '/vol/tmp/translate-2747021839723325609/5ae2de121ced560fc658f4c5.step', '2018-04-27T08:23:47', ( '' ), ( '' ), ' ', ' ', ' ' );
            FILE_SCHEMA( ( 'AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }' ) );
        ENDSEC;
        "#.trim();
        let (_residual, records) = crate::parser::exchange::header_section(&header)
            .finish()
            .unwrap();
        let header = super::Header::from_records(&records).unwrap();
        dbg!(header);
    }
}
