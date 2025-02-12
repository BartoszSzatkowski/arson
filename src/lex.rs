use crate::token::Token;

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
            peek_position: 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.input.len() {
            let tok = Token::try_from(self.input[self.position]).unwrap();
            self.position += 1;
            Some(tok)
        } else {
            None
        }
    }
}
