use crate::ast::{Decl, EnumDecl, TypeDecl, TypeDeclField};
use crate::lexing::{Token, TokenKind, TokenKindTag};

use super::error::Error;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Decl>, Error> {
        let mut decls = Vec::new();

        while let Some(kind) = self.peek() {
            if kind == &TokenKind::TypeDecl {
                decls.push(Decl::Type(self.parse_type_decl()?));
            } else if kind == &TokenKind::EnumDecl {
                decls.push(Decl::Enum(self.parse_enum_decl()?));
            } else {
                unreachable!()
            }
        }

        Ok(decls)
    }

    fn parse_type_decl(&mut self) -> Result<TypeDecl, Error> {
        self.eat(TokenKindTag::TypeDecl)?;
        let name = self.take_next_ident()?;
        self.eat(TokenKindTag::LBrace)?;

        if self.peek_blank_def() {
            self.eat(TokenKindTag::RBrace)?;
            return Ok(TypeDecl::new(name, Vec::new()));
        }

        let fields = self.parse_type_decl_fields()?;
        self.eat(TokenKindTag::RBrace)?;

        Ok(TypeDecl::new(name, fields))
    }

    fn parse_type_decl_fields(&mut self) -> Result<Vec<TypeDeclField>, Error> {
        let mut fields = Vec::new();

        loop {
            let field_name = self.take_next_ident()?;
            let field_ty = self.take_next_ident()?;

            fields.push(TypeDeclField::new(field_name, field_ty.as_str().into()));

            if matches!(self.peek(), Some(kind) if kind.tag() == TokenKindTag::RBrace) {
                break;
            }

            self.eat(TokenKindTag::Comma)?;
        }

        Ok(fields)
    }

    fn parse_enum_decl(&mut self) -> Result<EnumDecl, Error> {
        self.eat(TokenKindTag::EnumDecl)?;
        let name = self.take_next_ident()?;
        self.eat(TokenKindTag::LBrace)?;

        if self.peek_blank_def() {
            self.eat(TokenKindTag::RBrace)?;
            return Ok(EnumDecl::new(name, Vec::new()));
        }

        let mut values = Vec::new();

        loop {
            let value = self.take_next_ident()?;
            values.push(value);

            if matches!(self.peek(),Some(kind) if kind.tag() == TokenKindTag::RBrace) {
                break;
            }

            self.eat(TokenKindTag::Comma)?;
        }

        self.eat(TokenKindTag::RBrace)?;

        Ok(EnumDecl::new(name, values))
    }

    fn peek_blank_def(&self) -> bool {
        matches!(self.peek(), Some(kind) if kind.tag() == TokenKindTag::RBrace)
    }

    fn take_next_ident(&mut self) -> Result<String, Error> {
        let token = self.eat(TokenKindTag::Ident)?;
        match token.kind() {
            TokenKind::Ident(ident) => Ok(ident.clone()),
            _ => unreachable!(),
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos)?;
        self.pos += 1;
        Some(token)
    }

    fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.pos).map(Token::kind)
    }

    fn eat(&mut self, expected: TokenKindTag) -> Result<&Token, Error> {
        let token = self.advance();
        match token {
            Some(token) if token.kind().tag() == expected => Ok(token),
            Some(other) => Err(Error::UnexpectedToken {
                expected,
                found: other.kind().clone(),
                line: other.line(),
                col: other.col(),
            }),
            None => Err(Error::Eof),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexing::Lexer;

    #[test]
    fn test_valid_tokens() {
        let source = "
            type User {
                name string,
                age u8,
                lifeState HumanLifeState
            }

            enum HumanLifeState {
                Alive,
                Dead
            }
        ";

        let tokens = Lexer::new(source).get_tokens().unwrap();
        assert!(Parser::new(tokens).parse().is_ok());
    }

    #[test]
    fn test_invalid_tokens() {
        let source = "
            type User {
                name String # need ','
                age u8
            }
        ";
        let tokens = Lexer::new(source).get_tokens().unwrap();
        assert_eq!(
            Parser::new(tokens).parse(),
            Err(Error::UnexpectedToken {
                expected: TokenKindTag::Comma,
                found: TokenKind::Ident("age".to_owned()),
                line: 4,
                col: 17
            })
        );
    }

    #[test]
    fn test_blank_type_and_enum_def() {
        let source = "
            type User { }

            enum State { }
        ";

        let tokens = Lexer::new(source).get_tokens().unwrap();
        Parser::new(tokens).parse().unwrap();
    }
}
