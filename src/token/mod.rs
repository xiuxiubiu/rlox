use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TokenType {
    /// Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,

    /// One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GraterEqual,
    Less,
    LessEqual,

    // comment
    Slash,

    /// Literals
    Identifier,
    String,
    Number,

    /// Keywords.
    And,
    Class,
    Else,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    False,
    Var,
    While,

    Eof,
}

lazy_static! {
    pub static ref SINGLE_CHARACTOR_HASHMAP: HashMap<char, TokenType> = {
        let mut m = HashMap::new();
        m.insert('(', TokenType::LeftParen);
        m.insert(')', TokenType::RightParen);
        m.insert('{', TokenType::RightParen);
        m.insert('}', TokenType::RightParen);
        m.insert(',', TokenType::Comma);
        m.insert('.', TokenType::Dot);
        m.insert('-', TokenType::Minus);
        m.insert('+', TokenType::Plus);
        m.insert(';', TokenType::Semicolon);
        m.insert('*', TokenType::Star);
        m
    };
    pub static ref ONE_OR_TWO_CHARACTORS_HASHMAP: HashMap<char, TokenType> = {
        let mut m = HashMap::new();
        m.insert('!', TokenType::Bang);
        m.insert('=', TokenType::Equal);
        m.insert('>', TokenType::Greater);
        m.insert('<', TokenType::Less);
        m
    };
    pub static ref ONE_CORRESPOND_TWO_CHARACTORS_HASHMAP: HashMap<TokenType, TokenType> = {
        let mut m = HashMap::new();
        m.insert(TokenType::Bang, TokenType::BangEqual);
        m.insert(TokenType::Equal, TokenType::EqualEqual);
        m.insert(TokenType::Greater, TokenType::GraterEqual);
        m.insert(TokenType::Less, TokenType::LessEqual);
        m
    };
    pub static ref KEYWORDS_HASHMAP: HashMap<&'static str, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);
        keywords
    };
}

#[derive(Debug, Clone)]
pub struct Object {}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Object,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Object, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn get_token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn get_lexeme(&self) -> String {
        self.lexeme.clone()
    }

    pub fn get_literal(&self) -> Object {
        self.literal.clone()
    }

    pub fn get_line(&self) -> usize {
        self.line
    }
}
