use program::Program;
mod program;
mod expression;
mod statement;

pub enum Node {
    Program(Program),
    Statement,
    Expression,
}


