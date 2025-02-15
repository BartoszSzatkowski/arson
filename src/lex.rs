use crate::token::{
    COLON, COMMA, DOT, EQUALS, LEFTBRACKET, LEFTPAREN, LESSTHAN, RIGHTBRACKET, RIGHTPAREN,
    SEMICOLON, Token,
};

pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    peek_position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            position: 0,
            peek_position: 1,
        }
    }

    pub fn increment_position(&mut self) {
        self.position += 1;
        self.peek_position += 1;
    }

    pub fn parse_ident(&mut self) -> Token {
        let mut ident = vec![];
        loop {
            let v = self.input[self.position];
            if (v as char).is_ascii_alphanumeric() || (v as char) == '_' {
                ident.push(v);
                if self.peek_position == self.input.len() {
                    break;
                }

                let peek = self.input[self.peek_position];
                if peek.is_ascii_alphanumeric() || (peek as char) == '_' {
                    self.increment_position();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Token::parse_ident(&ident)
    }

    pub fn parse_number(&mut self) -> Token {
        let mut digits = vec![];
        loop {
            let v = self.input[self.position];
            if (v as char).is_ascii_digit() || (v as char) == '_' {
                digits.push(v);
                if self.peek_position == self.input.len() {
                    break;
                }

                let peek = self.input[self.peek_position];
                if peek.is_ascii_digit() || (peek as char) == '_' {
                    self.increment_position();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        let numeric_string: String = digits
            .into_iter()
            .map(|c| (c as char).to_string())
            .collect();

        Token::Number(numeric_string.parse::<i64>().unwrap())
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.input.len() {
            let tok = match &[self.input[self.position]] {
                LEFTPAREN => Token::LeftParen,
                RIGHTPAREN => Token::RightParen,
                LEFTBRACKET => Token::LeftBracket,
                RIGHTBRACKET => Token::RightBracket,
                COLON => Token::Colon,
                SEMICOLON => Token::Semicolon,
                EQUALS => Token::Equals,
                COMMA => Token::Comma,
                LESSTHAN => Token::LessThan,
                DOT => Token::Dot,
                [v] if (*v as char).is_whitespace() => Token::Whitespace,
                [v] if (*v as char).is_ascii_alphabetic() => self.parse_ident(),
                [v] if (*v as char).is_ascii_digit() => self.parse_number(),
                t => panic!("Cannot parse the token {:?}", t),
            };
            self.increment_position();

            Some(tok)
        } else {
            None
        }
    }
}
