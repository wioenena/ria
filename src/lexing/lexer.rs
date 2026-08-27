use super::TokenKind;
use super::error::Error;
use super::token::Token;

pub struct Lexer<'a> {
    source: &'a str,
    line: usize,
    col: usize,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            line: 1,
            col: 1,
            pos: 0,
        }
    }

    pub fn get_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();

        loop {
            if let Some(token) = self.next_token() {
                tokens.push(token?);
            } else {
                break;
            }
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Option<Result<Token, Error>> {
        self.skip_whitespace_and_comments();

        let (line, col) = (self.line, self.col);

        let kind = match self.peek() {
            Some(c) => match c {
                c if c.is_alphabetic() => self.read_ident_or_keyword(),
                '{' => {
                    self.advance();
                    TokenKind::LBrace
                }
                '}' => {
                    self.advance();
                    TokenKind::RBrace
                }
                ',' => {
                    self.advance();
                    TokenKind::Comma
                }
                _ => {
                    unreachable!();
                }
            },
            None => return None,
        };

        Some(Ok(Token::new(kind, line, col)))
    }

    fn read_ident_or_keyword(&mut self) -> TokenKind {
        let start = self.pos;

        while matches!(self.peek(), Some(c) if c.is_alphabetic() || c.is_ascii_digit()) {
            self.advance();
        }

        let text = &self.source[start..self.pos];

        match text {
            "type" => TokenKind::TypeDecl,
            "enum" => TokenKind::EnumDecl,
            _ => TokenKind::Ident(text.to_string()),
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.pos)?;
        self.pos += 1;

        if c == '\n' {
            self.line += 1;
            self.col = 1
        } else {
            self.col += 1;
        }

        Some(c)
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.pos)
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some(c) if c.is_ascii_whitespace() => {
                    self.advance();
                }
                Some('#') => {
                    while self.peek() != Some('\n') {
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static SOURCE: &str = "
        type User {
            name string,
            age u8,
            lifeState HumanLifeState
        }

        # this is comment.

        enum HumanLifeState {
            Alive,
            Dead
        }
    ";

    #[test]
    fn test_type_and_enum_decl() {
        let mut lexer = Lexer::new(SOURCE);
        let tokens = lexer
            .get_tokens()
            .unwrap()
            .iter()
            .clone()
            .map(|t| t.kind().clone())
            .collect::<Vec<_>>();

        assert_eq!(
            tokens,
            vec![
                TokenKind::TypeDecl,
                TokenKind::Ident("User".to_owned()),
                TokenKind::LBrace,
                TokenKind::Ident("name".to_owned()),
                TokenKind::Ident("string".to_owned()),
                TokenKind::Comma,
                TokenKind::Ident("age".to_owned()),
                TokenKind::Ident("u8".to_owned()),
                TokenKind::Comma,
                TokenKind::Ident("lifeState".to_owned()),
                TokenKind::Ident("HumanLifeState".to_owned()),
                TokenKind::RBrace,
                TokenKind::EnumDecl,
                TokenKind::Ident("HumanLifeState".to_owned()),
                TokenKind::LBrace,
                TokenKind::Ident("Alive".to_owned()),
                TokenKind::Comma,
                TokenKind::Ident("Dead".to_owned()),
                TokenKind::RBrace
            ]
        );
    }
}
