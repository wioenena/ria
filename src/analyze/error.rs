use crate::lexing::{TokenKind, TokenKindTag};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unexpected token {found:?} at {line}:{col}, expected: {expected:?}")]
    UnexpectedToken {
        expected: TokenKindTag,
        found: TokenKind,
        line: usize,
        col: usize,
    },
    #[error("Unexpected EOF")]
    Eof,
    #[error(transparent)]
    Type(#[from] crate::types::Error),
}
