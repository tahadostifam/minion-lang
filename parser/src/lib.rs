use ast::{
    expression::{
        BinaryExpression, Expression, Identifier, Integer, Literal, StringType, UnaryExpression,
    },
    program::Program,
    statement::Statement,
    Node,
};
use lexer::Lexer;
use precedences::{determine_token_precedence, Precedence};
use token::{Span, Token, TokenKind};

mod parser_test;
mod precedences;

type ParseError = String;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    // Init Parser
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let current_token = lexer
            .next_token()
            .expect("An error raised when reading current_token by lexer at parser");
        let peek_token = lexer
            .next_token()
            .expect("An error raised when reading peek_token by lexer at parser");

        let parser = Parser {
            lexer,
            current_token,
            peek_token,
            errors: vec![],
        };

        return parser;
    }

    // Public methods
    pub fn parse(input: &str) -> Result<Node, Vec<ParseError>> {
        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program()?;
        Ok(Node::Program(program))
    }

    // Private functionallities
    fn parse_program(&mut self) -> Result<Program, Vec<ParseError>> {
        let mut program = Program::new();

        while self.current_token.kind != TokenKind::EOF {
            match self.parse_statement() {
                Ok(statement) => program.body.push(statement),
                Err(error) => self.errors.push(error),
            }

            self.next_token();
        }

        if self.errors.len() > 0 {
            return Err(self.errors.clone());
        }

        return Ok(program);
    }

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
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.current_token.kind {
            // TODO - Implement IF STATEMENT
            // TODO - Implement ELSE STATEMENT
            // TODO - ...
            _ => self.parse_expression_statement(),
        }
    }

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
                    return Ok((
                        left,
                        Span {
                            start: left_start,
                            end: self.current_token.span.end,
                        },
                    ))
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
        let span = self.current_token.span.clone();

        let expr = match &self.current_token.kind {
            TokenKind::Identifier { name } => Expression::Identifier(Identifier {
                name: name.clone(),
                span,
            }),
            TokenKind::Integer(value) => Expression::Literal(Literal::Integer(Integer {
                raw: value.clone(),
                span,
            })),
            TokenKind::String(value) => Expression::Literal(Literal::String(StringType {
                raw: value.clone(),
                span,
            })),
            TokenKind::Minus => {
                let start = self.current_token.span.start;
                let prefix_operator = self.current_token.clone();

                self.next_token(); // consume the prefix operator

                let (expr, span) = self.parse_expression(Precedence::Prefix)?;

                Expression::Prefix(UnaryExpression {
                    operator: prefix_operator,
                    operand: Box::new(expr),
                    span: Span {
                        start,
                        end: span.end,
                    },
                })
            }

            _ => {
                return Err(format!(
                    "no prefix function found for the token: {}",
                    self.current_token.kind
                ));
            } // TODO - Implement boolean type here
              // TODO - Implement IF STATEMENT
              // TODO - Implement ELSE STATEMENT
              // TODO - Implement FUNCTION STATEMENT
              // TODO - Implement LEFT PARENT
              // TODO - Implement LEFT BRACKET
              // TODO - Implement LEFT BREACE
        };

        return Ok(expr);
    }

    fn parse_infix_expression(
        &mut self,
        left: Expression,
        left_start: usize,
    ) -> Option<Result<Expression, ParseError>> {
        match self.peek_token.kind {
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Asterisk
            | TokenKind::Slash
            | TokenKind::Modulo 
            | TokenKind::Equal
            | TokenKind::NotEqual
            | TokenKind::LessEqual
            | TokenKind::LessThan
            | TokenKind::GreaterEqual
            | TokenKind::GreaterThan
            => {
                self.next_token(); // consume the first part of the expression

                let operator = self.current_token.clone();
                let precedence = determine_token_precedence(self.current_token.kind.clone());

                self.next_token(); // consume the operator if the expression

                let (right, span) = self.parse_expression(precedence).unwrap();

                Some(Ok(Expression::Infix(BinaryExpression {
                    operator,
                    left: Box::new(left),
                    right: Box::new(right),
                    span: Span {
                        start: left_start,
                        end: span.end,
                    },
                })))
            }
            // TODO - Implement function call expression parser
            // TODO - Implement array index epxression parser
            _ => None,
        }
    }
}
