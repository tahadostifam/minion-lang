#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;

    #[test]
    fn test_parser_simple_expression() {
        let program = Parser::parse("hello").unwrap();

        println!("{:#?}", program);
    }
}
