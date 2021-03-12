use crate::{header::InvalidHeader, parser::TokenizeFailed};

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
}
