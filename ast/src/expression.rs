use core::fmt;

use token::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier,
    Literal,
    Prefix,
    Infix,
    Function,
    FunctionCall,
}

pub struct Identifier {
    pub name: String,
    pub span: Span,
}

pub enum Literal {
    Integer(Integer),
    Boolean(Boolean),
    String(StringType),
    Array(Array),
    Hash(Hash),
}

pub struct Integer {
    pub raw: i64,
    pub span: Span,
}

pub struct Boolean {
    pub raw: bool,
    pub span: Span,
}

pub struct StringType {
    pub raw: String,
    pub span: Span,
}

pub struct Array {
    pub elements: Vec<Expression>,
    pub span: Span,
}

pub struct Hash {
    pub pairs: Vec<(Expression, Expression)>,
    pub span: Span,
}
