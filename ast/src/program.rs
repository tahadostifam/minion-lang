use token::Span;

use crate::statement::Statement;

pub struct Program {
    pub body: Vec<Statement>,
    pub span: Span,
}

impl Program {
    pub fn new() -> Self {
        Self { body: vec![], span: Span::new_empty_span() }
    }
}
