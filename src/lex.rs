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

    pub fn curr_pos_byte(&self) -> Option<u8> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    pub fn peek_pos_byte(&self) -> Option<u8> {
        if self.peek_position < self.input.len() {
            Some(self.input[self.peek_position])
        } else {
            None
        }
    }

    pub fn parse_ident(&mut self, first_byte: u8) -> Token {
        let mut ident = vec![first_byte];
        self.increment_position();

        loop {
            let Some(c) = self.curr_pos_byte() else {
                break;
            };

            if (c as char).is_ascii_alphanumeric()
                || (c as char) == '_'
                || ((c as char) == '-'
                    && !matches!(self.peek_pos_byte(), Some(b) if (b as char) == '-'))
            {
                ident.push(c);
                self.increment_position();
            } else {
                break;
            }
        }

        Token::parse_ident(&ident)
    }

    pub fn parse_number(&mut self, first_byte: u8) -> Token {
        let mut digits = vec![first_byte];
        self.increment_position();

        loop {
            let Some(d) = self.curr_pos_byte() else {
                break;
            };
            if (d as char).is_ascii_digit() {
                digits.push(d);
                self.increment_position();
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

    fn eat_whitespace(&mut self) {
        self.increment_position();

        loop {
            if matches!(self.curr_pos_byte(), Some(b) if (b as char).is_ascii_whitespace()) {
                self.increment_position();
            } else {
                break;
            }
        }
    }

    fn eat_comment(&mut self) {
        self.increment_position();
        self.increment_position();

        loop {
            let Some(b) = self.curr_pos_byte() else {
                break;
            };

            if b == b'\n' {
                self.increment_position();
                break;
            }
            if b == b'-' && matches!(self.peek_pos_byte(), Some(p) if p == b'-') {
                self.increment_position();
                self.increment_position();
                break;
            }

            self.increment_position();
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.input.len() {
            loop {
                let mut has_eaten = false;

                let b = self.curr_pos_byte()?;
                if (b as char).is_ascii_whitespace() {
                    self.eat_whitespace();
                    has_eaten = true;
                }

                let b = self.curr_pos_byte()?;
                if b == b'-' && matches!(self.peek_pos_byte(), Some(p) if p == b'-') {
                    self.eat_comment();
                    has_eaten = true;
                }

                if !has_eaten {
                    break;
                }
            }

            let b = self.curr_pos_byte()?;
            let tok = match b {
                COLON => {
                    self.increment_position();
                    Token::Colon
                }
                COMMA => {
                    self.increment_position();
                    Token::Comma
                }
                DOT => {
                    self.increment_position();
                    Token::Dot
                }
                EQUALS => {
                    self.increment_position();
                    Token::Equals
                }
                LEFTBRACKET => {
                    self.increment_position();
                    Token::LeftBracket
                }
                LEFTPAREN => {
                    self.increment_position();
                    Token::LeftParen
                }
                LESSTHAN => {
                    self.increment_position();
                    Token::LessThan
                }
                RIGHTBRACKET => {
                    self.increment_position();
                    Token::RightBracket
                }
                RIGHTPAREN => {
                    self.increment_position();
                    Token::RightParen
                }
                SEMICOLON => {
                    self.increment_position();
                    Token::Semicolon
                }
                v if (v as char).is_ascii_alphabetic() => self.parse_ident(v),
                v if (v as char).is_ascii_digit() || v == b'-' => self.parse_number(v),
                t => panic!("Cannot parse the token {:?}", t),
            };

            Some(tok)
        } else {
            None
        }
    }
}
