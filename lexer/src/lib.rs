use std::fmt::{self, Debug};
use token::Token;

mod lexer_test;
mod token;

#[derive(Debug, Clone)]
pub struct Lexer {
    input: String,
    pos: usize,
    next_pos: usize,
    ch: char,
    errors: Vec<String>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input: input,
            pos: 0,      // points to current position
            next_pos: 0, // points to next position
            ch: ' ',
            errors: vec![],
        };

        lexer.read_char();

        return lexer;
    }

    fn peek_char(&self) -> char {
        if self.next_pos >= self.input.len() {
            return ' ';
        } else {
            self.input
                .chars()
                .nth(self.next_pos)
                .expect("Failed to read the char with peeked position")
        }
    }

    // The most significant section of the lexer is this method
    // that reads the char with next_pos and returns it.
    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = ' ';
        } else {
            self.ch = self
                .input
                .chars()
                .nth(self.next_pos)
                .expect("Failed to read the char with current position");
        }

        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        self.skip_comments();

        let matched_token = match self.ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '%' => Token::Modulo,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ',' => Token::Comma,
            '#' => Token::Hashtag,
            '"' => Token::DoubleQuote,
            '|' => Token::Pipe,
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char(); // consume current equal sign
                    self.read_char(); // consume peeked equal sign
                    return Ok(Token::Equal);
                } else {
                    self.read_char(); // consume current equal sign
                    return Ok(Token::Assign);
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    return Ok(Token::NotEqual);
                } else {
                    self.read_char(); // consume current equal sign
                    return Ok(Token::Bang);
                }
            }
            ' ' | '\0' => Token::EOF,
            _ => {
                return Err(format!("Illegal character detected {}", self.ch));
            }
        };

        self.read_char();
        Ok(matched_token)
    }

    fn skip_whitespace(&mut self) {
        if self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn skip_comments(&mut self) {
        if self.ch == '/' && self.peek_char() == '/' {
            self.read_char();
            self.read_char(); // consume double slash

            loop {
                // lets consume the comment :>
                if self.ch == '\n' || self.ch == '\u{0}' || self.pos == self.input.len() {
                    // consume the comments end
                    if self.ch == '\n' {
                        self.read_char();
                    }

                    break;
                }

                self.read_char(); // consume
            }
        }
    }

    fn is_number(src: String) -> bool {
        let c: u32 = src.chars().nth(0).unwrap() as u32;
        let bounds = ('1' as u32, '9' as u32);

        return c >= bounds.0 && c <= bounds.1;
    }

    fn is_alpha(src: String) -> bool {
        return src.to_lowercase() != src.to_uppercase();
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Err(str) => panic!("{}", str),
            Ok(token) => {
                if token == Token::EOF {
                    return None;
                }

                Some(token)
            }
        }
    }
}

impl fmt::Display for Lexer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "pos: {}, next_pos: {}, char: {}",
            self.pos, self.next_pos, self.ch
        )
    }
}
