use crate::{error::*, parser::*};
use std::path::PathBuf;

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
    name: PathBuf,
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
    pub fn from_records(records: &[Record]) -> Result<Self> {
        todo!()
    }
}
