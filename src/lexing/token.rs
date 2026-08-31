use super::token_kind::TokenKind;

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    line: usize,
    column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}
