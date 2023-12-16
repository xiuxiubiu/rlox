use rlox::token::{Object, Token, TokenType};

pub mod keywrods_test;

pub fn tokens_assert_eq(source: Vec<Token>, target: Vec<Token>) {
    for (index, token) in source.iter().enumerate() {
        let other_token = target.get(index).unwrap();
        assert_eq!(token.get_token_type(), other_token.get_token_type());
        assert_eq!(token.get_lexeme(), other_token.get_lexeme());
        assert_eq!(token.get_line(), other_token.get_line());
    }
}

#[test]
fn tokens_assert_eq_test() {
    let source = vec![
        Token::new(TokenType::Number, String::from("1"), Object {}, 1),
        Token::new(TokenType::And, String::from("and"), Object {}, 1),
        Token::new(TokenType::Eof, String::from(""), Object {}, 1),
    ];
    let target = vec![
        Token::new(TokenType::Number, String::from("1"), Object {}, 1),
        Token::new(TokenType::And, String::from("and"), Object {}, 1),
        Token::new(TokenType::Eof, String::from(""), Object {}, 1),
    ];
    tokens_assert_eq(source, target);
}
