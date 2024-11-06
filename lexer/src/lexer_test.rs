#[cfg(test)]
mod tests {
    use crate::Lexer;
    use token::TokenKind;

    fn assert_tokens(input: &str, expected_tokens: Vec<TokenKind>) {
        let lexer = Lexer::new(input.to_string());

        let mut i: usize = 0;
        for token in lexer {
            println!("{:?}", token);
            assert_eq!(token.kind, expected_tokens[i]);
            i += 1;
        }
    }

    #[test]
    fn test_operators() {
        assert_tokens(
            "+ - * / % =",
            vec![
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Asterisk,
                TokenKind::Slash,
                TokenKind::Modulo,
                TokenKind::Assign,
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
        assert_tokens("// Comment\n++", vec![TokenKind::Plus, TokenKind::Plus]);
    }

    #[test]
    fn test_symbols() {
        assert_tokens(
            "() {} , # \" |",
            vec![
                TokenKind::LeftParen,
                TokenKind::RightParen,
                TokenKind::LeftBrace,
                TokenKind::RightBrace,
                TokenKind::Comma,
                TokenKind::Hashtag,
                TokenKind::DoubleQuote,
                TokenKind::Pipe,
            ],
        );
    }

    #[test]
    fn test_equals() {
        assert_tokens(
            "!= , ==",
            vec![TokenKind::NotEqual, TokenKind::Comma, TokenKind::Equal],
        );
    }

    #[test]
    fn test_keywords() {
        assert_tokens(
            "fn match if else ret for break continue",
            vec![
                TokenKind::Function,
                TokenKind::Match,
                TokenKind::If,
                TokenKind::Else,
                TokenKind::Return,
                TokenKind::For,
                TokenKind::Break,
                TokenKind::Continue,
            ],
        );
    }

    #[test]
    fn test_less_greaters() {
        assert_tokens("<= >=", vec![TokenKind::LessEqual, TokenKind::GreaterEqual]);
    }

    #[test]
    fn test_and_or() {
        assert_tokens("&& ||", vec![TokenKind::And, TokenKind::Or]);
    }

    #[test]
    fn test_reading_identifier() {
        assert_tokens(
            "fn foo() {}",
            vec![
                TokenKind::Function,
                TokenKind::Identifier {
                    name: "foo".to_string(),
                },
                TokenKind::LeftParen,
                TokenKind::RightParen,
                TokenKind::LeftBrace,
                TokenKind::RightBrace,
            ],
        );
    }

    #[test]
    fn test_reading_random_identifiers() {
        assert_tokens(
            "hello world",
            vec![
                TokenKind::Identifier {
                    name: "hello".to_string(),
                },
                TokenKind::Identifier {
                    name: "world".to_string(),
                },
            ],
        );
    }

    #[test]
    fn testread_integer() {
        assert_tokens(
            "123 456",
            vec![TokenKind::Integer(123), TokenKind::Integer(456)],
        );
    }
}
