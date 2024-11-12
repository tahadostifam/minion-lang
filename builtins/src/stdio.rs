use object::object::Object;
use std::rc::Rc;

pub fn builtin_func_print(params: Vec<Rc<Object>>) -> Rc<Object> {
    params.iter().for_each(|obj| {
        println!("{}", obj);
    });

    Rc::from(Object::Null)
}

pub fn builtin_func_clear_screen(_params: Vec<Rc<Object>>) -> Rc<Object> {
    print!("\x1B[2J\x1B[1;1H");

    Rc::from(Object::Null)
}
