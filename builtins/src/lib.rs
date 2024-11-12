use lazy_static::lazy_static;
use object::object::BuiltinFunc;
use std::collections::HashMap;

mod stdio;

lazy_static! {
    pub static ref BuiltIns: BuiltinHashTable = BuiltinLoader::new();
}

type BuiltinHashTable = HashMap<&'static str, BuiltinFunc>;

struct BuiltinLoader {}

impl BuiltinLoader {
    pub fn new() -> BuiltinHashTable {
        let mut ht: BuiltinHashTable = HashMap::new();
        BuiltinLoader::load_funcs(&mut ht);
        return ht;
    }

    fn load_funcs(ht: &mut BuiltinHashTable) {
        ht.insert("print", stdio::builtin_func_print);
        ht.insert("clear", stdio::builtin_func_clear_screen);
    }
}
