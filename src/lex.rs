use crate::token::{Token, COLON, COMMA, DOT, EQUALS, LEFTBRACKET, LEFTPAREN, LESSTHAN, RIGHTBRACKET, RIGHTPAREN, SEMICOLON};

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
                [v] if (*v as char).is_ascii() => {
                    let mut ident = vec![];
                    loop { 
                        let v = self.input[self.position];
                        if (v as char).is_ascii_alphabetic() {
                            ident.push(v);
                            if self.input[self.peek_position].is_ascii_alphabetic() {
                                self.increment_position();
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    Token::Ident(ident)
                },
                t => panic!("Cannot parse the token {:?}", t),
            };
            self.increment_position();

            Some(tok)
        } else {
            None
        }
    }
}
