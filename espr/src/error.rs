use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Duplicate Entity ({name}) found in {schema} schema")]
    DuplicatedEntity { schema: String, name: String },

    #[error("Duplicate Schema ({name})")]
    DuplicatedSchema { name: String },
}
