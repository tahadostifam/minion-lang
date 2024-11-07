#[cfg(test)]
mod tests {
    use crate::Parser;

    #[test]
    fn test_parser_simple_expression() {
        Parser::parse("1 == 11").unwrap();
        Parser::parse("1 != 1").unwrap();
        Parser::parse("1 < 1").unwrap();
        Parser::parse("1 > 1").unwrap();
        Parser::parse("100 <= 100").unwrap();
        Parser::parse("100 >= 100").unwrap();
        Parser::parse("1 + 2").unwrap();
        Parser::parse("1 - 2").unwrap();
        Parser::parse("1 * 2").unwrap();
        Parser::parse("1 / 2").unwrap();
        Parser::parse("1 + 2 / 2").unwrap();
        Parser::parse("1 + 2 / 2 - 10").unwrap();
    }

    #[test]
    fn test_parse_bool_expressions() {
        Parser::parse("true").unwrap();
        Parser::parse("false").unwrap();
        Parser::parse("true == true").unwrap();
        Parser::parse("false == false").unwrap();
        Parser::parse("true == false").unwrap();
        Parser::parse("false == true").unwrap();
    }
}
