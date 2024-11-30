#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Eof,

    Comma,
    LeftParen,
    Quote,
    RightParen,
    Semicolon,

    Number(i16),
    String(String),
    Variable(char),

    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Minus,
    NotEqual,
    Plus,
    Slash,
    Star,

    // Keywords
    Clear,
    End,
    Gosub,
    Goto,
    If,
    Input,
    Let,
    List,
    Print,
    Rem,
    Return,
    Run,
    Then,
}
