#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("invalid attribute key: {0}")]
    InvalidAttributeKey(String),
    #[error("invalid attribute target: {0}")]
    InvalidTarget(String),
}
