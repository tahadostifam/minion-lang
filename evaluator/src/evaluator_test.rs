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
                    Err(e) => assert_eq!(&format!("{}", e), expected),
                },
                Err(e) => panic!("parse error: {}", e[0]),
            }
        }
    }

    #[test]
    fn test_integer_expressions() {
        let env: Env = Rc::new(RefCell::new(Default::default()));

        let program = Parser::parse("1 + 2".to_string()).unwrap();

        let result = eval(program, &env).unwrap();

        dbg!(result);
        // assert_eval(&[
        //     ("1 + 1", "0")
        // ]);
    }
}
