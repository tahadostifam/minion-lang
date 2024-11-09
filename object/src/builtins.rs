use std::rc::Rc;

use lazy_static::lazy_static;
use crate::object::{BuiltinFunc, Object};

lazy_static! {
    pub static ref BuiltIns: Vec<(&'static str, BuiltinFunc)> = vec![
        ("print", buildin_func_print)
    ];
}

pub fn buildin_func_print(params: Vec<Rc<Object>>)  -> Rc<Object> {
    params.iter().for_each(|obj| {
        println!("{}", obj);
    });
    Rc::from(Object::Null)
}
