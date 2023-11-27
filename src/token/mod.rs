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
    Slash,
    Star,

    /// One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Creater,
    GraterEqual,
    Less,
    LessEqual,

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
