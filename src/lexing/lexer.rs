use super::TokenKind;
use super::error::Error;
use super::token::Token;

pub struct Lexer<'a> {
    source: &'a str,
    line: usize,
    column: usize,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            line: 1,
            column: 1,
            pos: 0,
        }
    }

    pub fn get_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token?);
        }

        tokens.push(Token::new(TokenKind::Eof, self.line, self.column));
        Ok(tokens)
    }

    fn next_token(&mut self) -> Option<Result<Token, Error>> {
        self.skip_whitespace_and_comments();

        let (line, col) = (self.line, self.column);

        let kind = {
            let c = self.peek()?;
            match c {
                c if c.is_alphabetic() => self.read_ident_or_keyword(),
                c if c.is_ascii_digit() => self.read_number().ok()?,
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
                '@' => {
                    self.advance();
                    TokenKind::At
                }
                '(' => {
                    self.advance();
                    TokenKind::LParen
                }
                ')' => {
                    self.advance();
                    TokenKind::RParen
                }
                '=' => {
                    self.advance();
                    TokenKind::Eq
                }
                '"' => {
                    self.advance();
                    self.read_string()
                }
                _ => {
                    return Some(Err(Error::UnknownCharacter {
                        character: c,
                        line,
                        column: col,
                    }));
                }
            }
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
            _ => TokenKind::Ident(text.to_owned()),
        }
    }

    fn read_string(&mut self) -> TokenKind {
        let start = self.pos;

        while matches!(self.peek(), Some(c) if c != '"') {
            self.advance();
        }

        let text = &self.source[start..self.pos];

        self.advance(); // skip last '"'
        TokenKind::String(text.to_owned())
    }

    fn read_number(&mut self) -> Result<TokenKind, Error> {
        let start = self.pos;

        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            self.advance();
        }

        let text = &self.source[start..self.pos];

        let number = text.parse::<u32>();

        Ok(match number {
            Ok(number) => TokenKind::Number(number),
            Err(_) => return Err(Error::InvalidNumber(text.to_owned())),
        })
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.pos)?;
        self.pos += 1;

        if c == '\n' {
            self.line += 1;
            self.column = 1
        } else {
            self.column += 1;
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
                    while matches!(self.peek(), Some(c) if c != '\n') {
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
    use crate::lexing::TokenKindTag;

    fn tags_is_matched(tokens: &[Token], expected: &[TokenKindTag]) {
        for tag in tokens.iter().map(|t| t.kind().tag()) {
            assert!(expected.contains(&tag));
        }
    }

    #[test]
    fn test_basic_syntax() {
        use TokenKindTag::*;
        let source = "
            type User { age u8 }
        ";

        let tokens = Lexer::new(source).get_tokens().unwrap();
        assert_eq!(tokens.len(), 6 + 1 /* +1 for EOF */);

        tags_is_matched(
            &tokens,
            &[TypeDecl, Ident, LBrace, Ident, Ident, RBrace, Eof],
        );
    }

    #[test]
    fn test_with_attributes() {
        use TokenKindTag::*;
        let source = "
            @typescript(indent = 4)

            type User @typescript(as = \"class\") { age u8 }
        ";
        let tokens = Lexer::new(source).get_tokens().unwrap();
        assert_eq!(tokens.len(), 21);

        tags_is_matched(
            &tokens,
            &[
                At, Ident, LParen, Ident, Eq, Number, RParen, TypeDecl, Ident, At, Ident, LParen,
                Ident, String, RParen, LBrace, Ident, Ident, RBrace, Eof,
            ],
        );
    }

    #[test]
    fn test_with_comments() {
        use TokenKindTag::*;

        let source = "
            type User {
                name string # name of user
            }
        ";

        let tokens = Lexer::new(source).get_tokens().unwrap();
        assert_eq!(tokens.len(), 7);

        tags_is_matched(
            &tokens,
            &[TypeDecl, Ident, LBrace, Ident, Ident, RBrace, Eof],
        );
    }
}
