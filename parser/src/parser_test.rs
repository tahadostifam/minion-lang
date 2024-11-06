#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;

    #[test]
    fn test_parser_simple_expression() {
        let input = "1 + 2 * 3".to_string();
        let mut parser = Parser::new(input);
        let program = parser.parse_program();

        println!("{:?}", program);
    }
}
