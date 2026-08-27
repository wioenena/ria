use crate::lexing::{TokenKind, TokenKindTag};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unexpected token: {found:?}, expected: {expected:?}")]
    UnexpectedToken {
        expected: TokenKindTag,
        found: TokenKind,
    },
    #[error("Unexpected EOF")]
    Eof,
    #[error(transparent)]
    Type(#[from] crate::types::Error),
}
