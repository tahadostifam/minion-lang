#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EOF,
    Identifier { name: String },
    Integer(i32),
    Str(String),

    // Operators
    Plus,
    Minus,
    Slash,
    Asterisk,
    Modulo,

    // Symbols
    Assign,
    Equal,
    NotEqual,
    Bang,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Hashtag,
    DoubleQuote,
    Pipe,
    Semicolon,

    // Keywords
    Function,
    Match,
    If, 
    Else,
    Return,
    For,
    Break,
    Continue,
}
    