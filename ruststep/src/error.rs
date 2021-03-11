use crate::parser::TokenizeFailed;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, Hash)]
pub enum TypeKind {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    TokenizeFailed(#[from] TokenizeFailed),

    #[error("Type mismatch at {entity}:{position}, expected={expected:?}, actual={actual:?}")]
    TypeMismatch {
        entity: String,
        position: u32,
        expected: TypeKind,
        actual: TypeKind,
    },
}
