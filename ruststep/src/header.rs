use crate::parser::*;
use std::convert::TryFrom;

#[derive(Debug, thiserror::Error)]
pub enum InvalidHeader {
    #[error("Required record is missing: {name}")]
    RequiredRecordMissing { name: &'static str },

    #[error("Unknown Record: {name}")]
    UnknownRecord { name: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileDescription {
    description: Vec<String>,
    implementation_level: String,
}

impl TryFrom<Record> for FileDescription {
    type Error = crate::error::Error;
    fn try_from(record: Record) -> Result<Self, Self::Error> {
        assert_eq!(record.name, "FILE_DESCRIPTION");
        todo!()
    }
}

/// STEP-file HEADER section
///
/// There is a schema for HEADER section,
/// but we do not generate this structure from it to simplify build process.
///
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Header {
    // file_description
    description: Vec<String>,
    implementation_level: String,

    // file_name
    name: String,
    time_stamp: String,
    author: Vec<String>,
    organization: Vec<String>,
    preprocessor_version: String,
    originating_system: String,
    authorization: String,

    // file_schema
    schema: Vec<String>,
}

impl Header {
    pub fn from_records(records: &[Record]) -> Result<Self, InvalidHeader> {
        if records.len() == 0 {
            return Err(InvalidHeader::RequiredRecordMissing {
                name: "FILE_DESCRIPTION",
            });
        }

        let desc = &records[0];
        if desc.name != "FILE_DESCRIPTION" {
            return Err(InvalidHeader::UnknownRecord {
                name: desc.name.clone(),
            });
        }

        todo!()
    }
}
