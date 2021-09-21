use serde::{de, ser};
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    TokenizeFailed(#[from] TokenizeFailed),

    #[error("Error while deserialize STEP struct: {0}")]
    DeserializeFailed(String),

    #[error("Lookup failed for #{0}")]
    UnknownEntity(u64),

    #[error("Entity ID #{0} is duplicated")]
    DuplicatedEntity(u64),

    #[error("Entity '{entity_name}' is not a member of the schema '{schema}'")]
    UnknownEntityName { entity_name: String, schema: String },
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Error::DeserializeFailed(msg.to_string())
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Error::DeserializeFailed(msg.to_string())
    }
}

/// Error while tokenizing STEP input
#[derive(Debug)]
pub struct TokenizeFailed {
    rendered_error: String,
}

impl fmt::Display for TokenizeFailed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(
            f,
            "Error while tokenizing STEP input\n{}",
            self.rendered_error
        )?;
        Ok(())
    }
}

impl std::error::Error for TokenizeFailed {}

impl TokenizeFailed {
    pub fn new(input: &str, err: nom::error::VerboseError<&str>) -> Self {
        TokenizeFailed {
            rendered_error: nom::error::convert_error(input, err),
        }
    }
}
