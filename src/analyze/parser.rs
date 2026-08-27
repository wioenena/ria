use std::ops::Deref;

use crate::lexing::{Token, TokenKind};

use super::error::Error;
use super::type_info::TypeInfo;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<TypeInfo> {
        Vec::new()
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos)?;
        self.pos += 1;
        Some(token)
    }

    fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.pos).map(Token::kind)
    }

    fn eat(&mut self, kind: TokenKind) -> Result<&Token, Error> {
        let token = self.advance();
        match token {
            Some(token) if token.kind() == &kind => Ok(token),
            Some(other) => Err(Error::UnexpectedToken {
                expected: kind,
                found: other.kind().clone(),
            }),
            None => Err(Error::Eof),
        }
    }
}
