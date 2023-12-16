use rlox::{
    scanner::{Scan, Scanner},
    token::{Object, Token, TokenType},
};

use super::tokens_assert_eq;

#[test]
fn keywords() {
    let source_text = "1 and 3";

    let target = vec![
        Token::new(TokenType::Number, String::from("1"), Object {}, 1),
        Token::new(TokenType::And, String::from("and"), Object {}, 1),
        Token::new(TokenType::Number, String::from("3"), Object {}, 1),
        Token::new(TokenType::Eof, String::from(""), Object {}, 1),
    ];

    let mut scanner = Scanner::new(source_text);
    scanner.scan().unwrap();

    println!("{:?}", scanner.tokens());

    tokens_assert_eq(scanner.tokens(), target);
}
