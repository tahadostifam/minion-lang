use lexer::Lexer;
use ast::{expression::Expression, program::Program, statement::Statement};
use precedences::Precedence;
use token::{Span, Token};

mod precedences;
mod parser_test;

type ParseError = String;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParseError>,
}

impl Parser {
    // Init Parser
    pub fn new(input: String) -> Self {
        let parser = Parser {
            lexer: Lexer::new(input),
            current_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: vec![],
        }; 

        return parser;
    }

    // Public methods
    pub fn parse_program(&mut self) -> Program {
        let program = Program::new();
        let statements: Vec<Statement> = vec![];

        while self.current_token != Token::EOF {
            let token = self.next_token();

            match token {
                Token::Hashtag => {},
                Token::Function => {},
                Token::For => {},
                Token::Break => {},
                Token::If => {},
                Token::Else => {},
                Token::Return => {},
                Token::Continue => {},
                Token::Match => {},
                _ => {
                    // Parse expression happens here


                }
            }

            println!("{:?}", token);
        }
        
        return program;
    }


    // Private functionallities 
    fn next_token(&mut self) -> Token {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token().expect("Failed to read next_token in parser"); 
        return self.peek_token.clone();
    }

    fn current_token_is(&self, token: Token) -> bool {
        self.current_token == token
    }

    fn peek_token_is(&self, token: Token) -> bool {
        self.peek_token == token
    }

    fn expect_peek(&mut self, token: Token) -> Result<(), ParseError> {
        self.next_token();

        if self.peek_token_is(token.clone()) {
            return Ok(());
        }

        return Err(format!("expected token: {}, but got :{}", token, self.peek_token));
    }

    // Parse statements
    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.parse_expression(Precedence::Lowest)?.0;
        
        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }

        Ok(Statement::Expression(expr))
    }

    // Parse expressions
    fn parse_expression(&mut self, precedence: Precedence) -> Result<(Expression, Span), ParseError> {
        todo!()
    }

}
