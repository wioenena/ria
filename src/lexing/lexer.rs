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
                _ => {
                    dbg!(c);
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
            "enum" => TokenKind::Enum,
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
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.advance();
        }
    }
}
