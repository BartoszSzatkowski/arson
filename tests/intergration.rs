use arson::{lex::Lexer, token::Token};

#[test]
fn test_lexer() {
    let input = b"([.,])";
    let l = Lexer::new(input);

    // for t in l {
    //     dbg!(t);
    // }
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
