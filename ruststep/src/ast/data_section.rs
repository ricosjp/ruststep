use super::*;

use crate::{error::*, parser::exchange::data_section};

/// `DATA` section in STEP file
#[derive(Debug, Clone, PartialEq)]
pub struct DataSection {
    /// Metadata
    pub meta: Vec<Parameter>,
    /// Each lines in data section
    pub entities: Vec<EntityInstance>,
}

impl std::str::FromStr for DataSection {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        use nom::Finish;
        let input = input.trim();
        let (_input, record) = data_section(input)
            .finish()
            .map_err(|err| TokenizeFailed::new(input, err))?;
        Ok(record)
    }
}
