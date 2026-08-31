use strum_macros::EnumDiscriminants;

#[derive(Debug, Clone, PartialEq, EnumDiscriminants)]
#[strum_discriminants(name(TokenKindTag))]
pub enum TokenKind {
    Ident(String),
    String(String),
    Number(u32),

    // Decls
    EnumDecl,
    TypeDecl,

    // Symbols
    Comma,
    LParen,
    RParen,
    LBrace,
    RBrace,
    At,
    Eq,

    Eof,
}

impl TokenKind {
    pub fn tag(&self) -> TokenKindTag {
        TokenKindTag::from(self)
    }
}
