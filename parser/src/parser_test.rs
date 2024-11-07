#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;

    #[test]
    fn test_parser_simple_expression() {
        let program = Parser::parse("1 == 1").unwrap();

        println!("{:#?}", program);
    }
}
