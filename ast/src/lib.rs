use program::Program;

pub mod expression;
pub mod program;
pub mod statement;

pub enum Node {
    Program(Program),
    Statement,
    Expression,
}
