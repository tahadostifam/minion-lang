use std::borrow::Borrow;

use ast::{expression::Expression, program::Program, statement::Statement};
use lexer::Lexer;
use precedences::{determine_token_precedence, Precedence};
use token::{Span, Token, TokenKind};

mod parser_test;
mod precedences;

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
            current_token: Token { kind: TokenKind::Illegal, span: Span { start: 0, end: 0 } },
            peek_token: Token { kind: TokenKind::Illegal, span: Span { start: 0, end: 0 } },
            errors: vec![],
        };

        return parser;
    }

    // Public methods
    pub fn parse_program(&mut self) -> Program {
        let program = Program::new();
        let statements: Vec<Statement> = vec![];

        while self.current_token.kind != TokenKind::EOF {
            let token = self.next_token();

            match token.kind {
                TokenKind::Hashtag => {}
                TokenKind::Function => {}
                TokenKind::For => {}
                TokenKind::Break => {}
                TokenKind::If => {}
                TokenKind::Else => {}
                TokenKind::Return => {}
                TokenKind::Continue => {}
                TokenKind::Match => {}
                _ => {
                    // Parse expression happens here

                    // TODO
                    self.parse_expression(Precedence::Lowest);
                }
            }

            println!("{:?}", token);
        }

        return program;
    }

    // Private functionallities
    fn next_token(&mut self) -> Token {
        self.current_token = self.peek_token.clone();
        self.peek_token = self
            .lexer
            .next_token()
            .expect("Failed to read next_token in parser");
        return self.peek_token.clone();
    }

    fn current_token_is(&self, token_kind: TokenKind) -> bool {
        self.current_token.kind == token_kind
    }

    fn peek_token_is(&self, token_kind: TokenKind) -> bool {
        self.peek_token.kind == token_kind
    }

    fn expect_peek(&mut self, token: Token) -> Result<(), ParseError> {
        self.next_token();

        if self.peek_token_is(token.kind.clone()) {
            return Ok(());
        }

        return Err(format!(
            "expected token: {}, but got :{}",
            token.kind, self.peek_token.kind
        ));
    }

    // Parse statements
    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.parse_expression(Precedence::Lowest)?.0;

        if self.peek_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        Ok(Statement::Expression(expr))
    }

    // Parse expressions
    fn parse_expression(
        &mut self,
        precedence: Precedence,
    ) -> Result<(Expression, Span), ParseError> {
        let mut left_start = self.current_token.span.start;
        let mut left = self.parse_prefix_expression()?;

        while self.current_token.kind != TokenKind::EOF
            && precedence < determine_token_precedence(self.peek_token.kind.clone())
        {
            match self.parse_infix_expression(left.clone(), left_start) {
                Some(infix) => {
                    left = infix?;

                    if let Expression::Infix(b) = left.clone() {
                        left_start = b.span.start;
                    }
                }
                None => {
                    return Ok((left, Span { start: left_start, end: self.current_token.span.end }))
                }
            }
        }

        let end = self.current_token.span.end;

        Ok((
            left,
            Span {
                start: left_start,
                end,
            },
        ))
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
        todo!()
    }

    fn parse_infix_expression(
        &mut self,
        left: Expression,
        left_start: usize,
    ) -> Option<Result<Expression, ParseError>> {
        todo!()
    }
}
