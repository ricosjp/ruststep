use crate::{header::InvalidHeader, parser::TokenizeFailed};
use serde::de;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Hash)]
pub enum TypeKind {
    // Primitive types in the exchange structure spec
    Integer,
    Real,
    String,
    Enumeration,
    List,

    // Defined in a schema
    Entity(String),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    TokenizeFailed(#[from] TokenizeFailed),

    #[error(transparent)]
    InvalidHeader(#[from] InvalidHeader),

    #[error(
        "Number of parameters mismatch: entity={entity}, expected={expected}, actual={actual}"
    )]
    ParameterSizeMismatch {
        entity: String,
        expected: u32,
        actual: u32,
    },

    #[error("Type mismatch at {entity}:{position}, expected={expected:?}, actual={actual:?}")]
    TypeMismatch {
        entity: String,
        position: u32,
        expected: TypeKind,
        actual: TypeKind,
    },

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
