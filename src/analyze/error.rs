use crate::lexing::TokenKind;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unexpected token: {found}, expected: {expected}")]
    UnexpectedToken {
        expected: TokenKind,
        found: TokenKind,
    },
    #[error("Unexpected EOF")]
    Eof,
}
