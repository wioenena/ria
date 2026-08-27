use std::str::FromStr;

use crate::lexing::{Token, TokenKind, TokenKindTag};
use crate::types::Type;

use super::decl::Decl;
use super::enum_decl::EnumDecl;
use super::error::Error;
use super::type_decl::{TypeDecl, TypeDeclField};

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

        loop {
            match self.peek() {
                Some(kind) => {
                    if kind == &TokenKind::TypeDecl {
                        decls.push(Decl::Type(self.parse_type_decl()?));
                    } else if kind == &TokenKind::EnumDecl {
                        decls.push(Decl::Enum(self.parse_enum_decl()?));
                    } else {
                        unreachable!()
                    }
                }
                None => return Ok(decls),
            }
        }

        Ok(decls)
    }

    fn parse_type_decl(&mut self) -> Result<TypeDecl, Error> {
        self.eat(TokenKindTag::TypeDecl)?;
        let name = self.take_next_ident()?;
        self.eat(TokenKindTag::LBrace)?;

        let fields = self.parse_type_decl_fields()?;
        self.eat(TokenKindTag::RBrace)?;

        let type_decl = TypeDecl::new(name, fields);

        Ok(type_decl)
    }

    fn parse_type_decl_fields(&mut self) -> Result<Vec<TypeDeclField>, Error> {
        let mut fields = Vec::new();

        loop {
            let field_name = self.take_next_ident()?;
            let field_ty = self.take_next_ident()?;

            fields.push(TypeDeclField::new(field_name, Type::from_str(&field_ty)?));

            match self.peek() {
                Some(kind) if kind.tag() == TokenKindTag::Comma => {
                    self.advance();
                    if let Some(next) = self.peek() {
                        if next.tag() == TokenKindTag::Ident {
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        unreachable!();
                    }
                }
                Some(_) => {
                    break;
                }
                None => unreachable!(),
            }
        }

        Ok(fields)
    }

    fn parse_enum_decl(&mut self) -> Result<EnumDecl, Error> {
        self.eat(TokenKindTag::EnumDecl)?;
        let name = self.take_next_ident()?;
        self.eat(TokenKindTag::LBrace)?;
        let mut values = Vec::new();

        loop {
            let value = self.take_next_ident()?;
            values.push(value);

            match self.peek() {
                Some(kind) if kind.tag() == TokenKindTag::Comma => {
                    self.advance();
                    if let Some(next) = self.peek() {
                        if next.tag() == TokenKindTag::Ident {
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        unreachable!();
                    }
                }
                Some(_) => break,
                None => unreachable!(),
            }
        }

        self.eat(TokenKindTag::RBrace)?;

        Ok(EnumDecl::new(name, values))
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

    fn peek_at(&self, offset: usize) -> Option<&TokenKind> {
        self.tokens.get(self.pos + offset).map(Token::kind)
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
    use crate::lexing::Lexer;

    use super::*;
    static SOURCE: &str = "
        type User {
            name string,
            age u8,
            # lifeState HumanLifeState
        }

        enum HumanLifeState {
            Alive,
            Dead
        }
    ";

    #[test]
    fn test_parse_type_and_enum_decls() {
        let mut lexer = Lexer::new(SOURCE);
        let tokens = lexer.get_tokens().unwrap();
        let mut parser = Parser::new(tokens);

        let decls = parser.parse().unwrap();

        assert_eq!(
            decls,
            vec![
                Decl::Type(TypeDecl::new(
                    "User".to_owned(),
                    vec![
                        TypeDeclField::new("name".to_owned(), Type::String),
                        TypeDeclField::new("age".to_owned(), Type::U8)
                    ]
                )),
                Decl::Enum(EnumDecl::new(
                    "HumanLifeState".to_owned(),
                    vec!["Alive".to_owned(), "Dead".to_owned()]
                ))
            ]
        );
    }
}
