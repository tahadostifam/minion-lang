#[cfg(test)]
mod tests {
    use crate::{token::Token, Lexer};

    #[test]
    fn test_operators() {
        let input = String::from("+ - * / % =");
        let lexer = Lexer::new(input);

        let detected_tokens: Vec<Token> = vec![
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::Slash,
            Token::Modulo,
            Token::Assign,
        ];

        let mut i: usize = 0;
        for token in lexer {
            assert_eq!(token, detected_tokens[i]);
            i += 1;
        }
    }

    #[test]
    fn test_comments() {
        let input = String::from("// This is a comment! :)");
        let mut lexer = Lexer::new(input);

        lexer.next_token().unwrap();
    }

    #[test]
    fn test_comments_and_operators() {
        let input = String::from("// Comment\n++");
        let lexer = Lexer::new(input);

        let detected_tokens: Vec<Token> = vec![Token::Plus, Token::Plus];

        let mut i: usize = 0;
        for token in lexer {
            assert_eq!(token, detected_tokens[i]);
            i += 1;
        }

        assert_eq!(i, detected_tokens.len());
    }

    #[test]
    fn test_symbols() {
        let input = String::from("() {} , # \" |");
        let lexer = Lexer::new(input);

        let detected_tokens: Vec<Token> = vec![
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::Hashtag,
            Token::DoubleQuote,
            Token::Pipe,
        ];

        println!("\n\n");

        let mut i: usize = 0;
        for token in lexer {
            assert_eq!(token, detected_tokens[i]);
            i += 1;
        }

        println!("\n\n");
    }

    #[test]
    fn test_equals() {
        let input = String::from("!= , ==");
        let lexer = Lexer::new(input);

        let detected_tokens: Vec<Token> = vec![Token::NotEqual, Token::Comma, Token::Equal];

        let mut i: usize = 0;
        for token in lexer {
            assert_eq!(token, detected_tokens[i]);
            i += 1;
        }
    }

    #[test]
    fn test_keywords() {
        let input = String::from("fn match if else ret for break continue");
        let lexer = Lexer::new(input);

        let detected_tokens: Vec<Token> = vec![
            Token::Function,
            Token::Match,
            Token::If,
            Token::Else,
            Token::Return,
            Token::For,
            Token::Break,
            Token::Continue,
        ];

        let mut i: usize = 0;
        for token in lexer {
            assert_eq!(token, detected_tokens[i]);
            i += 1;
        }
    }

    #[test]
    fn test_reading_identifier() {
        let lexer = Lexer::new("fn foo() {}".to_string());

        let detected_tokens: Vec<Token> = vec![
            Token::Function,
            Token::Identifier { name: "foo".to_string() },
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
        ];

        let mut i: usize = 0;
        for token in lexer {
            assert_eq!(token, detected_tokens[i]);
            i += 1;
        }
    }

    #[test]
    fn test_reading_random_identifiers() {
        let lexer = Lexer::new("hello world".to_string());

        let detected_tokens: Vec<Token> = vec![
            Token::Identifier { name: "hello".to_string() },
            Token::Identifier { name: "world".to_string() },
        ];

        let mut i: usize = 0;
        for token in lexer {
            assert_eq!(token, detected_tokens[i]);
            i += 1;
        }
    }

    #[test]
    fn test_is_numeric() {
        let lexer = Lexer::new("".to_string());
        assert_eq!(lexer.is_numeric('1'), true);
        assert_eq!(lexer.is_numeric('9'), true);
        assert_eq!(lexer.is_numeric('a'), false);
    }
}
