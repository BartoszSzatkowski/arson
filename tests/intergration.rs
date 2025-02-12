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
    let input = b" abcd ";
    let l = Lexer::new(input);

    assert_eq!(
        vec![
            Token::Whitespace,
            Token::Ident(b"abcd".to_vec()),
            Token::Whitespace,
            Token::Dot,
        ],
        l.into_iter().collect::<Vec<_>>()
    )
}
