#[cfg(test)]
mod tests {
    use parser::Parser;
    use std::{cell::RefCell, rc::Rc};

    use object::env::Env;

    use crate::eval;

    fn assert_eval(test_cases: &[(&str, &str)]) {
        let env: Env = Rc::new(RefCell::new(Default::default()));
        for (input, expected) in test_cases {
            match Parser::parse(input.to_string()) {
                Ok(node) => match eval(node, &env) {
                    Ok(evaluated) => assert_eq!(&format!("{}", evaluated), expected),
                    Err(e) => assert_eq!(&e.to_string(), expected),
                },
                Err(e) => panic!("parse error: {}", e[0]),
            }
        }
    }

    #[test]
    fn test_integer_expressions() {
        assert_eval(&[
            ("1 + 1", "2"),
            ("(1 + 1) + 1", "3"),
            ("10 / 2 + 1", "6"),
            ("5 + (1 + 1) - 10", "-3"),
            // ("10 % 2", "0")
        ]);
    }


    #[test]
    fn test_function_declaration() {
        assert_eval(&[
            ("fn foo_bar(a, b) { ret a + b; }", "null")
        ]);
    }
}
