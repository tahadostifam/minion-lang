use std::fmt;
use token::{Span, Token};

use crate::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(Variable),
    Expression(Expression),
}

pub fn format_statements(stmts: &Vec<Statement>) -> String {
    stmts.iter().map(|stmt| stmt.to_string()).collect()
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub identifier: Token,
    pub expr: Expression,
    pub span: Span,
}
