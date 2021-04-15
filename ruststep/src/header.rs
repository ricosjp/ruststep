use crate::{ast::*, error::Result};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FileDescription {
    description: Vec<String>,
    implementation_level: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FileName {
    name: String,
    time_stamp: String,
    author: Vec<String>,
    organization: Vec<String>,
    preprocessor_version: String,
    originating_system: String,
    authorization: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FileSchema {
    schema: Vec<String>,
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
