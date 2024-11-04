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
    Bang,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Hashtag,
    DoubleQuote,
    Pipe,

    // Keywords
    Disco,
}
