#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown character: {character}, line: {line}, column: {column}")]
    UnknownCharacter {
        character: char,
        line: usize,
        column: usize,
    },
    #[error("unknown keyword: {0}")]
    UnknownKeyword(String),
    #[error("invalid number: {0}")]
    InvalidNumber(String),
}
