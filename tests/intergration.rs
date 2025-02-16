use arson::{lex::Lexer, token::Token};

#[test]
fn parses_asn1_special_characters() {
    let input = b"([.,])";
    let l = Lexer::new(input);

    assert_eq!(
        vec![
            Token::LeftParen,
            Token::LeftBracket,
            Token::Dot,
            Token::Comma,
            Token::RightBracket,
            Token::RightParen,
        ],
        l.into_iter().collect::<Vec<_>>()
    )
}

#[test]
fn parses_idents() {
    let input = b" ab_cd ";
    let l = Lexer::new(input);

    assert_eq!(
        vec![
            Token::Whitespace,
            Token::Ident(b"ab_cd".to_vec()),
            Token::Whitespace,
        ],
        l.into_iter().collect::<Vec<_>>()
    )
}

#[test]
fn parses_keywords() {
    let input = b"DEFINITIONS BEGIN FROM";
    let l = Lexer::new(input);

    assert_eq!(
        vec![
            Token::Definitions,
            Token::Whitespace,
            Token::Begin,
            Token::Whitespace,
            Token::From
        ],
        l.into_iter().collect::<Vec<_>>()
    )
}

#[test]
fn parses_numbers() {
    let input = b"123";
    let l = Lexer::new(input);

    assert_eq!(
        vec![Token::Number(123_i64),],
        l.into_iter().collect::<Vec<_>>()
    )
}

#[test]
fn parses_negative_numbers() {
    let input = b"-123";
    let l = Lexer::new(input);

    assert_eq!(
        vec![Token::Number(-123_i64),],
        l.into_iter().collect::<Vec<_>>()
    )
}
