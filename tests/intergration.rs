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
        vec![Token::Ident(b"ab_cd".to_vec()),],
        l.into_iter().collect::<Vec<_>>()
    )
}

#[test]
fn parses_keywords() {
    let input = b"DEFINITIONS BEGIN FROM";
    let l = Lexer::new(input);

    assert_eq!(
        vec![Token::Definitions, Token::Begin, Token::From],
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

#[test]
fn ignores_whitespace() {
    let input = b"   123   abc OF\n\t";
    let l = Lexer::new(input);

    assert_eq!(
        vec![
            Token::Number(123_i64),
            Token::Ident(b"abc".to_vec()),
            Token::Of,
        ],
        l.into_iter().collect::<Vec<_>>()
    )
}

#[test]
fn ignores_comments() {
    let input = b"123 --comment--abc";
    let l = Lexer::new(input);

    assert_eq!(
        vec![Token::Number(123_i64), Token::Ident(b"abc".to_vec()),],
        l.into_iter().collect::<Vec<_>>()
    )
}

#[test]
fn terminates_comments_at_the_end_of_the_line() {
    let input = b"123 abc--\n123";
    let l = Lexer::new(input);

    assert_eq!(
        vec![
            Token::Number(123_i64),
            Token::Ident(b"abc".to_vec()),
            Token::Number(123_i64)
        ],
        l.into_iter().collect::<Vec<_>>()
    )
}

#[test]
fn ignores_comments_at_the_end_of_the_file() {
    let input = b"123 --comment--abc--";
    let l = Lexer::new(input);

    assert_eq!(
        vec![Token::Number(123_i64), Token::Ident(b"abc".to_vec()),],
        l.into_iter().collect::<Vec<_>>()
    )
}

#[test]
fn ignores_comments_with_dashes_inside() {
    let input = b"123 -- also a -- --comment--abc--";
    let l = Lexer::new(input);

    assert_eq!(
        vec![Token::Number(123_i64), Token::Ident(b"abc".to_vec()),],
        l.into_iter().collect::<Vec<_>>()
    )
}

#[test]
fn ignores_comments_at_multiple_lines() {
    let input = b"-- This is a comment
-- on multiple lines -- 123";
    let l = Lexer::new(input);

    assert_eq!(
        vec![Token::Number(123_i64)],
        l.into_iter().collect::<Vec<_>>()
    )
}
