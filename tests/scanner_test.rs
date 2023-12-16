mod scanner;

use rlox::scanner::{Scan, Scanner};

#[test]
fn scanner_bang_equal() {
    let mut scanner = Scanner::new("!= *");
    scanner.scan().unwrap();
    println!("{:?}", scanner.tokens());
    assert_eq!(scanner.tokens().len(), 3);
}

#[test]
fn scanner_equal() {
    let mut scanner = Scanner::new("!(");
    scanner.scan().unwrap();
    println!("{:?}", scanner.tokens());
    assert_eq!(scanner.tokens().len(), 3);
}

#[test]
fn comment() {
    let mut scanner = Scanner::new(
        r#"// 你好
                                    !="#,
    );
    scanner.scan().unwrap();
    println!("{:?}", scanner.tokens());
    assert_eq!(scanner.tokens().len(), 2);
}

#[test]
fn string() {
    let mut scanner = Scanner::new("!= \"你好\"");
    scanner.scan().unwrap();
    println!("{:?}", scanner.tokens());
    assert_eq!(scanner.tokens().len(), 3);
}

#[test]
#[should_panic]
fn unterminated_string() {
    let mut scanner = Scanner::new("!= \"你好");
    scanner.scan().unwrap();
}

#[test]
fn number() {
    let mut scanner = Scanner::new("!= 1.23 123 .123 123.");
    scanner.scan().unwrap();
    println!("{:?}", scanner.tokens());
    assert_eq!(scanner.tokens().len(), 8);
}
