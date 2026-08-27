#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ident(String),

    // Keywords
    Enum,

    // Symbols
    LBrace,
    RBrace,
}
