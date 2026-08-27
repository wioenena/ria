#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid type: {0}")]
    InvalidType(String),
}
