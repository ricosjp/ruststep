use crate::{ast::*, error::Result, tables::Holder};
use ruststep_derive::{as_holder, Holder};
use serde::{de, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct HeaderTable {
    file_description: HashMap<u64, as_holder!(FileDescription)>,
    file_name: HashMap<u64, as_holder!(FileName)>,
    file_schema: HashMap<u64, as_holder!(FileSchema)>,
}

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = HeaderTable, field = file_description)]
pub struct FileDescription {
    description: Vec<String>,
    implementation_level: String,
}

impl<'de> de::Deserialize<'de> for FileDescription {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let holder = <as_holder!(FileDescription)>::deserialize(deserializer)?;
        let table = HeaderTable::default();
        Ok(holder.into_owned(&table).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = HeaderTable, field = file_name)]
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
        let holder = <as_holder!(FileName)>::deserialize(deserializer)?;
        let table = HeaderTable::default();
        Ok(holder.into_owned(&table).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Holder)]
#[holder(table = HeaderTable, field = file_schema)]
pub struct FileSchema {
    schema: Vec<String>,
}

impl<'de> de::Deserialize<'de> for FileSchema {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let holder = <as_holder!(FileSchema)>::deserialize(deserializer)?;
        let table = HeaderTable::default();
        Ok(holder.into_owned(&table).unwrap())
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
