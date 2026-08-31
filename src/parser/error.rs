use crate::lexing::{TokenKind, TokenKindTag};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("unexpected token {found:?} at line: {line}, column: {column}, expected: {expected:?}")]
    UnexpectedToken {
        expected: Vec<TokenKindTag>,
        found: TokenKind,
        line: usize,
        column: usize,
    },
    #[error("{source} at {line}:{column}")]
    Ast {
        #[source]
        source: crate::ast::attribute::Error,
        line: usize,
        column: usize,
    },
    #[error("unexpected EOF")]
    Eof,
}
