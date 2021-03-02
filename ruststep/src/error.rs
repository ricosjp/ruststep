pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse STEP file:\n{0}")]
    ParseFailed(String),
}
