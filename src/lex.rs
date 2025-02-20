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
            if (v as char).is_ascii_digit() || v == b'-' {
                digits.push(v);
                if self.peek_position == self.input.len() {
                    break;
                }

                let peek = self.input[self.peek_position];
                if peek.is_ascii_digit() {
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

    fn eat_whitespace(&mut self) -> Option<()> {
        self.increment_position();
        loop {
            if self.position >= self.input.len() {
                // EOF
                return None;
            }
            if (self.input[self.position] as char).is_ascii_whitespace() {
                self.increment_position();
            } else {
                // More tokens remaning
                return Some(());
            }
        }
    }

    fn eat_comment(&mut self) -> Option<()> {
        self.increment_position();
        loop {
            if self.peek_position >= self.input.len() {
                // EOF
                return None;
            }
            if self.input[self.position] == b'\n' {
                self.increment_position();
                if self.position >= self.input.len() {
                    return None;
                } else {
                    return Some(());
                }
            }

            if self.input[self.position] == b'-' && self.input[self.peek_position] == b'-' {
                self.increment_position();
                self.increment_position();
                if self.position >= self.input.len() {
                    return None;
                } else {
                    return Some(());
                }
            } else {
                self.increment_position();
            }
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.input.len() {
            loop {
                if self.position >= self.input.len() {
                    return None;
                }
                let eating_whitespace_result =
                    if (self.input[self.position] as char).is_ascii_whitespace() {
                        self.eat_whitespace()
                    } else {
                        Some(())
                    };
                if self.position >= self.input.len() {
                    return None;
                }
                let eating_comment_result = if self.input[self.position] == b'-'
                    && self.input[self.peek_position] == b'-'
                {
                    self.eat_comment()
                } else {
                    Some(())
                };

                if eating_whitespace_result.is_none() || eating_comment_result.is_none() {
                    return None;
                }

                if eating_whitespace_result.is_some() && eating_comment_result.is_some() {
                    break;
                }
            }
            let tok = match self.input[self.position] {
                COLON => Token::Colon,
                COMMA => Token::Comma,
                DOT => Token::Dot,
                EQUALS => Token::Equals,
                LEFTBRACKET => Token::LeftBracket,
                LEFTPAREN => Token::LeftParen,
                LESSTHAN => Token::LessThan,
                RIGHTBRACKET => Token::RightBracket,
                RIGHTPAREN => Token::RightParen,
                SEMICOLON => Token::Semicolon,
                v if (v as char).is_ascii_alphabetic() => self.parse_ident(),
                v if (v as char).is_ascii_digit() || v == b'-' => self.parse_number(),
                t => panic!("Cannot parse the token {:?}", t),
            };
            self.increment_position();

            Some(tok)
        } else {
            None
        }
    }
}
