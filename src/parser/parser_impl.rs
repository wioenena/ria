use std::collections::HashMap;
use std::str::FromStr;

use crate::ast::attribute::{
    AttributeContainer, AttributeField, AttributeKey, AttributeTargetLanguage, AttributeUsedIn,
};
use crate::ast::{Decl, EnumDecl, Program, TypeDecl, TypeDeclField, Value};
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

    pub fn parse(&mut self) -> Result<Program, Error> {
        let mut program = Program::new(Vec::new());

        while let Some(kind) = self.peek() {
            if kind == &TokenKind::TypeDecl {
                program.add_decl(Decl::Type(self.parse_type_decl()?));
            } else if kind == &TokenKind::EnumDecl {
                program.add_decl(Decl::Enum(self.parse_enum_decl()?));
            } else if kind == &TokenKind::At {
                self.parse_attributes(program.attribute_container_mut(), AttributeUsedIn::Root)?;
            } else if kind == &TokenKind::Eof {
                break;
            } else {
                unreachable!()
            }
        }

        Ok(program)
    }

    fn parse_type_decl(&mut self) -> Result<TypeDecl, Error> {
        let type_decl_token = self.eat(TokenKindTag::TypeDecl)?;
        let (line, column) = (type_decl_token.line(), type_decl_token.column());

        let (_, _, name) = self.take_next_ident()?;
        let mut attribute_container = AttributeContainer::new();

        if matches!(self.peek(), Some(kind) if kind.tag() == TokenKindTag::At) {
            self.parse_attributes(&mut attribute_container, AttributeUsedIn::Type)?;
        }

        self.eat(TokenKindTag::LBrace)?;

        if self.peek_blank_def() {
            self.eat(TokenKindTag::RBrace)?;
            return Ok(TypeDecl::new(
                name,
                Vec::new(),
                attribute_container,
                line,
                column,
            ));
        }

        let fields = self.parse_type_decl_fields()?;
        self.eat(TokenKindTag::RBrace)?;

        Ok(TypeDecl::new(
            name,
            fields,
            attribute_container,
            line,
            column,
        ))
    }

    fn parse_type_decl_fields(&mut self) -> Result<Vec<TypeDeclField>, Error> {
        let mut fields = Vec::new();

        loop {
            let (line, column, field_name) = self.take_next_ident()?;
            let (_, _, field_ty) = self.take_next_ident()?;
            let mut attribute_container = AttributeContainer::new();

            if matches!(self.peek(), Some(kind) if kind.tag() == TokenKindTag::At) {
                self.parse_attributes(&mut attribute_container, AttributeUsedIn::TypeField)?;
            }

            fields.push(TypeDeclField::new(
                field_name,
                field_ty.as_str().into(),
                attribute_container,
                line,
                column,
            ));

            if matches!(self.peek(), Some(kind) if kind.tag() == TokenKindTag::RBrace) {
                break;
            }

            self.eat(TokenKindTag::Comma)?;
        }

        Ok(fields)
    }

    fn parse_enum_decl(&mut self) -> Result<EnumDecl, Error> {
        let type_decl_token = self.eat(TokenKindTag::EnumDecl)?;
        let (line, column) = (type_decl_token.line(), type_decl_token.column());

        let (_, _, name) = self.take_next_ident()?;
        self.eat(TokenKindTag::LBrace)?;

        if self.peek_blank_def() {
            self.eat(TokenKindTag::RBrace)?;
            return Ok(EnumDecl::new(
                name,
                Vec::new(),
                HashMap::new(),
                line,
                column,
            ));
        }

        let mut values = Vec::new();

        loop {
            let (_, _, value) = self.take_next_ident()?;
            values.push(value);

            if matches!(self.peek(),Some(kind) if kind.tag() == TokenKindTag::RBrace) {
                break;
            }

            self.eat(TokenKindTag::Comma)?;
        }

        self.eat(TokenKindTag::RBrace)?;

        Ok(EnumDecl::new(name, values, HashMap::new(), line, column))
    }

    fn parse_attributes(
        &mut self,
        container: &mut AttributeContainer,
        used_in: AttributeUsedIn,
    ) -> Result<(), Error> {
        while matches!(self.peek(), Some(kind) if kind.tag() == TokenKindTag::At) {
            self.eat(TokenKindTag::At)?; // Skip '@'

            let field_name_token = self.eat(TokenKindTag::Ident)?;

            let TokenKind::Ident(target_name) = field_name_token.kind() else {
                unreachable!()
            };

            let target =
                AttributeTargetLanguage::from_str(target_name.as_str()).map_err(|source| {
                    Error::Ast {
                        source,
                        line: field_name_token.line(),
                        column: field_name_token.column(),
                    }
                })?;

            self.eat(TokenKindTag::LParen)?;

            while matches!(self.peek(), Some(kind) if kind.tag() != TokenKindTag::RParen) {
                let field_name_token = self.eat(TokenKindTag::Ident)?;
                let (line, column) = (field_name_token.line(), field_name_token.column());

                let TokenKind::Ident(field_name) = field_name_token.kind() else {
                    unreachable!()
                };

                let field_name = field_name.clone();

                self.eat(TokenKindTag::Eq)?;

                let value_token = self.eat_one_of(&[TokenKindTag::String, TokenKindTag::Number])?;

                let value = match value_token.kind() {
                    TokenKind::String(v) => Value::String(v.clone()),
                    TokenKind::Number(v) => Value::Number(*v),
                    _ => unreachable!(),
                };

                if let Some(kind) = self.peek()
                    && kind.tag() == TokenKindTag::Comma
                {
                    self.eat(TokenKindTag::Comma)?;
                }

                let key =
                    AttributeKey::from_str(field_name.as_str()).map_err(|source| Error::Ast {
                        source,
                        line,
                        column,
                    })?;

                container.add_field(
                    target,
                    key,
                    AttributeField::new(key, value, used_in, line, column),
                );
            }

            self.eat(TokenKindTag::RParen)?;
        }

        Ok(())
    }

    fn peek_blank_def(&self) -> bool {
        matches!(self.peek(), Some(kind) if kind.tag() == TokenKindTag::RBrace)
    }

    fn take_next_ident(&mut self) -> Result<(usize, usize, String), Error> {
        let token = self.eat(TokenKindTag::Ident)?;
        match token.kind() {
            TokenKind::Ident(ident) => Ok((token.line(), token.column(), ident.clone())),
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
                expected: vec![expected],
                found: other.kind().clone(),
                line: other.line(),
                column: other.column(),
            }),
            None => Err(Error::Eof),
        }
    }

    fn eat_one_of(&mut self, expected: &[TokenKindTag]) -> Result<&Token, Error> {
        let token = self.advance();
        match token {
            Some(token) if expected.contains(&token.kind().tag()) => Ok(token),
            Some(other) => Err(Error::UnexpectedToken {
                expected: expected.to_vec(),
                found: other.kind().clone(),
                line: other.line(),
                column: other.column(),
            }),
            None => Err(Error::Eof),
        }
    }
}
