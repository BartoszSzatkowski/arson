// Special ASN characters : ; = , < . ( ) [ ]
pub const COLON: u8 = b':';
pub const COMMA: u8 = b',';
pub const DOT: u8 = b'.';
pub const EQUALS: u8 = b'=';
pub const LEFTBRACKET: u8 = b'[';
pub const LEFTPAREN: u8 = b'(';
pub const LESSTHAN: u8 = b'<';
pub const RIGHTBRACKET: u8 = b']';
pub const RIGHTPAREN: u8 = b')';
pub const SEMICOLON: u8 = b';';
// Keywords
pub const ANY: &[u8] = b"ANY";
pub const APPLICATION: &[u8] = b"APPLICATION";
pub const ASSIGNMENT: &[u8] = b"::=";
pub const BEGIN: &[u8] = b"BEGIN";
pub const BIT: &[u8] = b"BIT";
pub const BOOLEAN: &[u8] = b"BOOLEAN";
pub const BY: &[u8] = b"BY";
pub const CHARACTER: &[u8] = b"CHARACTER";
pub const CHOICE: &[u8] = b"CHOICE";
pub const CLASS: &[u8] = b"CLASS";
pub const COMPONENTS: &[u8] = b"COMPONENTS";
pub const CONSTRAINED: &[u8] = b"CONSTRAINED";
pub const DEFAULT: &[u8] = b"DEFAULT";
pub const DEFINED: &[u8] = b"DEFINED";
pub const DEFINITIONS: &[u8] = b"DEFINITIONS";
pub const END: &[u8] = b"END";
pub const ENUMERATED: &[u8] = b"ENUMERATED";
pub const EXPLICIT: &[u8] = b"EXPLICIT";
pub const EXPORTS: &[u8] = b"EXPORTS";
pub const FALSE: &[u8] = b"FALSE";
pub const FROM: &[u8] = b"FROM";
pub const IMPLICIT: &[u8] = b"IMPLICIT";
pub const IMPORTS: &[u8] = b"IMPORTS";
pub const INSTANCE: &[u8] = b"INSTANCE";
pub const NULL: &[u8] = b"NULL";
pub const OBJECT: &[u8] = b"OBJECT";
pub const OCTET: &[u8] = b"OCTET";
pub const OF: &[u8] = b"OF";
pub const OPTIONAL: &[u8] = b"OPTIONAL";
pub const PATTERN: &[u8] = b"PATTERN";
pub const PRIVATE: &[u8] = b"PRIVATE";
pub const REAL: &[u8] = b"REAL";
pub const SEQUENCE: &[u8] = b"SEQUENCE";
pub const SET: &[u8] = b"SET";
pub const SIZE: &[u8] = b"SIZE";
pub const STRING: &[u8] = b"STRING";
pub const SYNTAX: &[u8] = b"SYNTAX";
pub const TRUE: &[u8] = b"TRUE";
pub const UNIQUE: &[u8] = b"UNIQUE";
pub const UNIVERSAL: &[u8] = b"UNIVERSAL";
pub const WITH: &[u8] = b"WITH";

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
    Ident(Vec<u8>),
    Number(i64), // assumming that numbers bigger that that does not apprear on the reagular basis
    // Keywords
    Any,
    Application,
    Assignment,
    Begin,
    Bit,
    Boolean,
    By,
    Character,
    Choice,
    Class,
    Components,
    Constrained,
    Default,
    Defined,
    Definitions,
    End,
    Enumerated,
    Explicit,
    Exports,
    False,
    From,
    Implicit,
    Imports,
    Instance,
    Null,
    Object,
    Octet,
    Of,
    Optional,
    Pattern,
    Private,
    Real,
    Sequence,
    Set,
    Size,
    String,
    Syntax,
    True,
    Unique,
    Universal,
    With,
}

impl Token {
    pub fn parse_ident(ident: &Vec<u8>) -> Self {
        match ident.as_slice() {
            ANY => Token::Any,
            APPLICATION => Token::Application,
            ASSIGNMENT => Token::Assignment,
            BEGIN => Token::Begin,
            BIT => Token::Bit,
            BOOLEAN => Token::Boolean,
            BY => Token::By,
            CHARACTER => Token::Character,
            CHOICE => Token::Choice,
            CLASS => Token::Class,
            COMPONENTS => Token::Components,
            CONSTRAINED => Token::Constrained,
            DEFAULT => Token::Default,
            DEFINED => Token::Defined,
            DEFINITIONS => Token::Definitions,
            END => Token::End,
            ENUMERATED => Token::Enumerated,
            EXPLICIT => Token::Explicit,
            EXPORTS => Token::Exports,
            FALSE => Token::False,
            FROM => Token::From,
            IMPLICIT => Token::Implicit,
            IMPORTS => Token::Imports,
            INSTANCE => Token::Instance,
            NULL => Token::Null,
            OBJECT => Token::Object,
            OCTET => Token::Octet,
            OF => Token::Of,
            OPTIONAL => Token::Optional,
            PATTERN => Token::Pattern,
            PRIVATE => Token::Private,
            REAL => Token::Real,
            SEQUENCE => Token::Sequence,
            SET => Token::Set,
            SIZE => Token::Size,
            STRING => Token::String,
            SYNTAX => Token::Syntax,
            TRUE => Token::True,
            UNIQUE => Token::Unique,
            UNIVERSAL => Token::Universal,
            WITH => Token::With,
            _ => Self::Ident(ident.to_vec()),
        }
    }
}
