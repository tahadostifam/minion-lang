use token::Token;

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

pub fn determine_token_precedence(token: &Token) -> Precedence {
    match token {
        Token::Equal => Precedence::Equals,
        Token::NotEqual => Precedence::Equals,
        Token::LessThan=> Precedence::LessGreater,
        Token::LessEqual=> Precedence::LessGreater,
        Token::GreaterThan=> Precedence::LessGreater,
        Token::GreaterEqual=> Precedence::LessGreater,
        Token::Plus => Precedence::Sum,
        Token::Minus => Precedence::Sum,
        Token::Asterisk => Precedence::Product,
        Token::Slash => Precedence::Product,
        Token::LeftParen => Precedence::Call,
        Token::LeftBracket => Precedence::Index,
        _ => Precedence::Lowest,
    }
}
