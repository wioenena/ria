#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ident(String),

    // Decls
    EnumDecl,
    TypeDecl,

    // Symbols
    Comma,
    LBrace,
    RBrace,
}

#[derive(Debug, PartialEq)]
pub enum TokenKindTag {
    Ident,

    // Decls
    EnumDecl,
    TypeDecl,

    // Symbols
    Comma,
    LBrace,
    RBrace,
}

impl TokenKind {
    pub const fn tag(&self) -> TokenKindTag {
        match self {
            TokenKind::Ident(_) => TokenKindTag::Ident,
            TokenKind::EnumDecl => TokenKindTag::EnumDecl,
            TokenKind::TypeDecl => TokenKindTag::TypeDecl,
            TokenKind::Comma => TokenKindTag::Comma,
            TokenKind::LBrace => TokenKindTag::LBrace,
            TokenKind::RBrace => TokenKindTag::RBrace,
        }
    }
}
