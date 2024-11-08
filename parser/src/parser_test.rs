#[cfg(test)]
mod tests {
    use lexer::Lexer;

    use crate::Parser;

    fn assert_parse(input: &str) {
        match Parser::parse(input) {
            Ok(program) => {
                println!("{:#?}", program);
            }
            Err(parse_errors) => {
                println!("{:#?}", parse_errors);
            }
        }
    }

    #[test]
    fn test_parser_simple_expression() {
        assert_parse("1 == 11");
        assert_parse("1 != 1");
        assert_parse("1 < 1");
        assert_parse("1 > 1");
        assert_parse("100 <= 100");
        assert_parse("100 >= 100");
        assert_parse("1 + 2");
        assert_parse("1 - 2");
        assert_parse("1 * 2");
        assert_parse("1 / 2");
        assert_parse("1 + 2 / 2");
        assert_parse("1 + 2 / 2 - 10");
    }

    #[test]
    fn test_parse_bool_expressions() {
        assert_parse("true");
        assert_parse("false");
        assert_parse("true == true");
        assert_parse("false == false");
        assert_parse("true == false");
        assert_parse("false == true");
    }

    #[test]
    fn test_variable_declaration() {
        assert_parse("#my_var = 1 + 2 * 3;");
    }

    #[test]
    fn test_if_statement() {
        assert_parse("if (1 < 2) { 3 } else {}");
    }

    #[test]
    fn test_return_statement() {
        assert_parse("ret 1 + 2");
    }

    #[test]
    fn test_parse_params() {
        let mut lexer = Lexer::new("(a, b, c)".to_string());
        let mut parser = Parser::new(&mut lexer);
        let params = parser.parse_params();

        println!("{:#?}", params);
    }

    #[test]
    fn test_function_statement() {
        assert_parse("fn foo_bar(a, b) { ret a + b; }");
    }
}
