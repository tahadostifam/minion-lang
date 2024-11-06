#[cfg(test)]
mod tests {
    use token::Token;
    use crate::Lexer;

    fn assert_tokens(input: &str, expected_tokens: Vec<Token>) {
        let lexer = Lexer::new(input.to_string());

        let mut i: usize = 0;
        for token in lexer {
            assert_eq!(token, expected_tokens[i]);
            i += 1;
        }
    }

    #[test]
    fn test_operators() {
        assert_tokens(
            "+ - * / % =",
            vec![
                Token::Plus,
                Token::Minus,
                Token::Asterisk,
                Token::Slash,
                Token::Modulo,
                Token::Assign,
            ],
        );
    }

    #[test]
    fn test_comments() {
        let input = String::from("// This is a comment! :)");
        let mut lexer = Lexer::new(input);

        lexer.next_token().unwrap();
    }

    #[test]
    fn test_comments_and_operators() {
        assert_tokens("// Comment\n++", vec![Token::Plus, Token::Plus]);
    }

    #[test]
    fn test_symbols() {
        assert_tokens(
            "() {} , # \" |",
            vec![
                Token::LeftParen,
                Token::RightParen,
                Token::LeftBrace,
                Token::RightBrace,
                Token::Comma,
                Token::Hashtag,
                Token::DoubleQuote,
                Token::Pipe,
            ],
        );
    }

    #[test]
    fn test_equals() {
        assert_tokens("!= , ==", vec![Token::NotEqual, Token::Comma, Token::Equal]);
    }

    #[test]
    fn test_keywords() {
        assert_tokens(
            "fn match if else ret for break continue",
            vec![
                Token::Function,
                Token::Match,
                Token::If,
                Token::Else,
                Token::Return,
                Token::For,
                Token::Break,
                Token::Continue,
            ],
        );
    }

    #[test]
    fn test_less_greaters() {
        assert_tokens(
            "<= >=",
            vec![
                Token::LessEqual,
                Token::GreaterEqual,
            ],
        );
    }
    
    #[test]
    fn test_and_or() {
        assert_tokens(
            "&& ||",
            vec![
                Token::And,
                Token::Or,
            ],
        );
    }

    #[test]
    fn test_reading_identifier() {
        assert_tokens(
            "fn foo() {}",
            vec![
                Token::Function,
                Token::Identifier {
                    name: "foo".to_string(),
                },
                Token::LeftParen,
                Token::RightParen,
                Token::LeftBrace,
                Token::RightBrace,
            ],
        );
    }

    #[test]
    fn test_reading_random_identifiers() {
        assert_tokens(
            "hello world",
            vec![
                Token::Identifier {
                    name: "hello".to_string(),
                },
                Token::Identifier {
                    name: "world".to_string(),
                },
            ],
        );
    }

    #[test]
    fn testread_integer() {
        assert_tokens("123 456", vec![Token::Integer(123), Token::Integer(456)]);
    }
}
