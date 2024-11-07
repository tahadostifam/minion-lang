use std::fmt;
use crate::expression::{Expression, Literal};

#[derive(Debug)]
pub enum Statement {
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