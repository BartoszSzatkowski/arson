use std::convert::TryFrom;

// Special ASN characters : ; = , < . ( ) [ ]
pub const LEFTPAREN: &[u8; 1] = b"(";
pub const RIGHTPAREN: &[u8; 1] = b")";
pub const LEFTBRACKET: &[u8; 1] = b"[";
pub const RIGHTBRACKET: &[u8; 1] = b"]";
pub const COLON: &[u8; 1] = b":";
pub const SEMICOLON: &[u8; 1] = b";";
pub const EQUALS: &[u8; 1] = b"=";
pub const COMMA: &[u8; 1] = b",";
pub const LESSTHAN: &[u8; 1] = b"<";
pub const DOT: &[u8; 1] = b".";
// Keywords
pub const DEFINITIONS: &[u8] = b"DEFINITIONS";
pub const SEQUENCE: &[u8] = b"SEQUENCE";
pub const BEGIN: &[u8] = b"BEGIN";
pub const END: &[u8] = b"END";
pub const ENUMERATED: &[u8] = b"ENUMERATED";

#[derive(Debug, PartialEq)]
pub enum Token {
    // Special ASN characters : ; = , < . ( ) [ ]
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Colon,
    Semicolon,
    Equals,
    Comma,
    LessThan,
    Dot,
    // Keywords
    Definitions,
    Sequence,
    Begin,
    End,
    Enumerated,
}

impl TryFrom<u8> for Token {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match &[value] {
            LEFTPAREN => Ok(Token::LeftParen),
            RIGHTPAREN => Ok(Token::RightParen),
            LEFTBRACKET => Ok(Token::LeftBracket),
            RIGHTBRACKET => Ok(Token::RightBracket),
            COLON => Ok(Token::Colon),
            SEMICOLON => Ok(Token::Semicolon),
            EQUALS => Ok(Token::Equals),
            COMMA => Ok(Token::Comma),
            LESSTHAN => Ok(Token::LessThan),
            DOT => Ok(Token::Dot),
            _ => Err(()),
        }
    }
}
