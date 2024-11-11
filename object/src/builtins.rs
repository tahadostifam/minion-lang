use std::rc::Rc;

use lazy_static::lazy_static;
use crate::object::{BuiltinFunc, Object};

lazy_static! {
    pub static ref BuiltIns: Vec<(&'static str, BuiltinFunc)> = vec![
        ("print", buildin_func_print),
        ("clear", buildin_func_clear_screen)
    ];
}

pub fn buildin_func_print(params: Vec<Rc<Object>>)  -> Rc<Object> {
    params.iter().for_each(|obj| {
        println!("{}", obj);
    });
    
    Rc::from(Object::Null)
}

pub fn buildin_func_clear_screen(_params: Vec<Rc<Object>>)  -> Rc<Object> {
    print!("\x1B[2J\x1B[1;1H");
    
    Rc::from(Object::Null)
}
