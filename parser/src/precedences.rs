use token::TokenKind;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // + or =
    Product,     // * or /
    Prefix,      // -X or !X
    Call,        // my_function(x)
    Index,       // array[index]
}

pub fn determine_token_precedence(token_kind: TokenKind) -> Precedence {
    match token_kind {
        TokenKind::Equal => Precedence::Equals,
        TokenKind::NotEqual => Precedence::Equals,
        TokenKind::LessThan => Precedence::LessGreater,
        TokenKind::LessEqual => Precedence::LessGreater,
        TokenKind::GreaterThan => Precedence::LessGreater,
        TokenKind::GreaterEqual => Precedence::LessGreater,
        TokenKind::Plus => Precedence::Sum,
        TokenKind::Minus => Precedence::Sum,
        TokenKind::Asterisk => Precedence::Product,
        TokenKind::Slash => Precedence::Product,
        TokenKind::LeftParen => Precedence::Call,
        TokenKind::LeftBracket => Precedence::Index,
        _ => Precedence::Lowest,
    }
}
