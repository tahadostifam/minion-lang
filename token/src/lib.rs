#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EOF,
    Identifier { name: String },
    Integer(i64),
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

// Span essentially pinpoints the token's exact location within the source file.
// That is really useful for error-reporting, syntax-hightlighting, and etc.
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn new_empty_span() -> Self {
        Self { start: 0, end: 0 }
    }
}
