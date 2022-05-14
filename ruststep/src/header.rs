//! Header section of exchange structure
//!
//! `HEADER` section of exchange structure shall contains
//! one instance of each of following entities in this order:
//!
//! - `file_description`
//! - `file_name`
//! - `file_schema`
//!
//! and following entities may appear after `file_schema`:
//!
//! - `schema_population`
//! - `file_population`
//! - `section_language`
//! - `section_context`
//!
//! These entities are defined in [ISO-10303-21 "8.2 Header section declarations"](https://www.iso.org/standard/63141.html)
//! using EXPRESS schemas.
//! Although we can generate corresponding Rust struct using espr compiler,
//! we write these definitions manually to keep development process simple.
//!

use crate::{ast::*, error::Result};
use serde::Deserialize;

/// File description
///
/// Following EXPRESS schema is an exerpt from
/// [ISO-10303-21:2016(E) "8.2.2 file_description"](https://www.iso.org/standard/63141.html):
///
/// ```text
/// ENTITY file_description;
///   description          : LIST [1:?] OF STRING (256) ;
///   implementation_level : STRING (256) ;
/// END_ENTITY;
/// ```
#[derive(Debug, Clone, PartialEq, ruststep_derive::Deserialize)]
pub struct FileDescription {
    pub description: Vec<String>,
    pub implementation_level: String,
}

/// File name
///
/// Following EXPRESS schema is an exerpt from
/// [ISO-10303-21:2016(E) "8.2.3 file_name"](https://www.iso.org/standard/63141.html):
///
/// ```text
/// ENTITY file_name;
///   name                 : STRING (256) ;
///   time_stamp           : time_stamp_text ;
///   author               : LIST [ 1 : ? ] OF STRING (256) ;
///   organization         : LIST [ 1 : ? ] OF STRING (256) ;
///   preprocessor_version : STRING (256) ;
///   originating_system   : STRING (256) ;
///   authorization        : STRING (256) ;
/// END_ENTITY;
///
/// TYPE time_stamp_text = STRING(256);
/// END_TYPE;
/// ```
#[derive(Debug, Clone, PartialEq, ruststep_derive::Deserialize)]
pub struct FileName {
    pub name: String,
    /// ISO-8601 formatted date and time specifying when the exchange structure was created.
    pub time_stamp: String,
    pub author: Vec<String>,
    pub organization: Vec<String>,
    pub preprocessor_version: String,
    pub originating_system: String,
    pub authorization: String,
}

/// File schema
///
/// Following EXPRESS schema is an exerpt from
/// [ISO-10303-21:2016(E) "8.2.4 file_schema"](https://www.iso.org/standard/63141.html):
///
/// ```text
/// ENTITY file_schema;
///   schema_identifiers : LIST [1:?] OF UNIQUE schema_name;
/// END_ENTITY;
///
/// TYPE schema_name = STRING(1024);
/// END_TYPE;
/// ```
#[derive(Debug, Clone, PartialEq, ruststep_derive::Deserialize)]
pub struct FileSchema {
    pub schema: Vec<String>,
}

/// STEP-file HEADER section
///
/// There is a schema for HEADER section,
/// but we do not generate this structure from it to simplify build process.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub file_description: FileDescription,
    pub file_name: FileName,
    pub file_schema: FileSchema,
}

impl Header {
    pub fn from_records(records: &[SimpleEntityInstance]) -> Result<Self> {
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
        let (_residual, records) = crate::parser::exchange::header_section(header)
            .finish()
            .unwrap();
        let header = super::Header::from_records(&records).unwrap();
        dbg!(header);
    }
}
