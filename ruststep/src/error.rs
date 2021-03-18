use crate::{header::InvalidHeader, parser::TokenizeFailed};
use serde::de;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    TokenizeFailed(#[from] TokenizeFailed),

    #[error(transparent)]
    InvalidHeader(#[from] InvalidHeader),

    #[error("Error while deserialize STEP struct: {0}")]
    DeserializeFailed(String),
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Error::DeserializeFailed(msg.to_string())
    }
}
