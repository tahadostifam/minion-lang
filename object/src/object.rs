use crate::env::Env;
use ast::{expression::Identifier, statement::BlockStatement};
use core::fmt;
use std::rc::Rc;

pub type EvalError = String;
pub type BuiltinFunc = fn(Vec<Rc<Object>>) -> Rc<Object>;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    ReturnValue(Rc<Object>),
    Function(Vec<Identifier>, BlockStatement, Env),
    Builtin(BuiltinFunc),
    Error(String),
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(v) => write!(f, "{}", v),
            Object::Boolean(v) => write!(f, "{}", v),
            Object::String(v) => write!(f, "{}", v),
            Object::Error(v) => write!(f, "{}", v),
            Object::ReturnValue(expr) => write!(f, "{}", expr),
            Object::Builtin(_) => write!(f, "[builtin func]"),
            Object::Function(params, body, .. ) => {
                let func_params = params
                    .iter()
                    .map(|stmt| stmt.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                write!(f, "fn({}) {{ {} }}", func_params, body)
            }
            Object::Null => write!(f, "null"),
        }
    }
}
