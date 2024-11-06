use crate::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Expression(Expression)
}