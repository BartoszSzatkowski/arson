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
    Whitespace,
    Ident(Vec<u8>),
    // Keywords
    Definitions,
    Sequence,
    Begin,
    End,
    Enumerated,
}
