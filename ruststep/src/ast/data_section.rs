use super::*;

use crate::{error::*, parser::exchange::data_section};
use std::str::FromStr;

/// `DATA` section in STEP file
#[derive(Debug, Clone, PartialEq)]
pub struct DataSection {
    /// Metadata
    pub meta: Vec<Parameter>,
    /// Each lines in data section
    pub entities: Vec<EntityInstance>,
}

impl FromStr for DataSection {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_section_from_str() {
        let input = r#"
        DATA;
          #1 = A(1.0, 2.0);
          #2 = B(3.0, A((4.0, 5.0)));
          #3 = B(6.0, #1);
        ENDSEC;
        "#
        .trim();
        let data_section = DataSection::from_str(input).unwrap();
        dbg!(data_section);
    }
}
