use ast::{
    expression::{
        BinaryExpression, Boolean, Expression, FunctionCall, Identifier, Integer, Literal, StringType, UnaryExpression
    },
    program::Program,
    statement::{BlockStatement, Function, If, Return, Statement, Variable},
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
    pub fn parse(input: String) -> Result<Node, Vec<ParseError>> {
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program()?;
        Ok(Node::Program(program))
    }

    // Parse statements (The main method of this struct actually)
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.current_token.kind {
            TokenKind::If => self.parse_if_statement(),
            TokenKind::Function => self.parse_function_statement(),
            TokenKind::Return => self.parse_return_statement(),
            TokenKind::Hashtag => self.parse_variable_declaration(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_token.span.start;
        self.next_token(); // consume sharp token

        let identifier = self.current_token.clone(); // export the name of the identifier

        self.expect_peek(TokenKind::Assign)?;

        let (expr, span) = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        Ok(Statement::VariableDeclaration(Variable {
            identifier,
            expr,
            span: Span {
                start,
                end: span.end,
            },
        }))
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

    fn expect_peek(&mut self, token_kind: TokenKind) -> Result<(), ParseError> {
        if self.peek_token_is(token_kind.clone()) {
            self.next_token(); // consume current token
            return Ok(());
        }

        return Err(format!(
            "expected token: {} but got {}",
            token_kind, self.peek_token.kind
        ));
    }

    fn expect_current(&mut self, token_kind: TokenKind) -> Result<(), ParseError> {
        if self.current_token_is(token_kind.clone()) {
            self.next_token(); // consume current token
            return Ok(());
        }

        return Err(format!(
            "expected token: {} but got {}",
            token_kind, self.current_token.kind
        ));
    }

    fn parse_function_params(&mut self) -> Result<Vec<Identifier>, ParseError> {
        self.expect_current(TokenKind::LeftParen)?;

        let mut params: Vec<Identifier> = Vec::new();

        while self.current_token.kind != TokenKind::RightParen {
            match self.current_token.kind.clone() {
                TokenKind::Identifier { name } => {
                    params.push(Identifier {
                        name,
                        span: self.current_token.span.clone(),
                    });

                    match &self.peek_token.kind {
                        TokenKind::Comma => {
                            self.next_token();
                        }
                        TokenKind::RightParen => {
                            self.next_token();
                            break;
                        }
                        _ => {
                            return Err(format!(
                                "expected a comma or the end of the parameters but got: {}",
                                self.current_token.kind
                            ))
                        }
                    }

                    self.next_token(); // consume the current identifier
                }
                _ => {
                    return Err(format!(
                        "expected an identifier set as paramater of the function but got: {}",
                        self.current_token.kind
                    ))
                }
            }
        }

        self.expect_current(TokenKind::RightParen)?;

        Ok(params)
    }

    // Parse statements
    fn parse_expression_series(
        &mut self,
        end: TokenKind,
    ) -> Result<(Vec<Expression>, Span), ParseError> {
        let start = self.current_token.span.start;
        let mut series: Vec<Expression> = Vec::new();

        // Detect empty series of expressions
        if self.peek_token_is(end.clone()) {
            self.next_token();

            return Ok((
                series,
                Span {
                    start,
                    end: self.current_token.span.end,
                },
            ));
        }

        self.next_token(); // consume the starting token

        series.push(self.parse_expression(Precedence::Lowest)?.0); // parse the first expression

        // !self.peek_token_is(end.clone())
        while self.peek_token_is(TokenKind::Comma) {
            self.next_token(); // consume the current expression

            if self.current_token_is(TokenKind::Comma) && self.peek_token_is(end.clone()) {
                self.next_token(); // consume last comma 
                break;
            }

            self.next_token(); // consume the comma

            series.push(self.parse_expression(Precedence::Lowest)?.0);
        }

        if self.peek_token_is(end.clone()) {
            self.next_token(); // consume the latest expression
        }

        if !self.current_token_is(end.clone()) {
            return Err(format!(
                "expected {} to close the expression series but got: {}",
                end, self.current_token.kind
            ))
        }

        Ok((
            series,
            Span {
                start,
                end: self.current_token.span.end,
            },
        ))
    }
    fn parse_function_statement(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_token.span.start;

        self.next_token(); // consume the fn token

        let function_name = match self.current_token.kind.clone() {
            TokenKind::Identifier { name } => name,
            _ => {
                return Err(format!(
                    "the name of the function can't be except an identifier but got: {}",
                    self.current_token.kind
                ))
            }
        }; // export the name of the function
        self.next_token(); // consume the name of the identifier

        let params = self.parse_function_params()?;

        // we used current_token_is because we don't want to consume it,
        // we pass this statement that is inside a brace to parse_block_statement.
        if self.current_token_is(TokenKind::LeftBrace) {
            let body = Box::new(self.parse_block_statement()?);

            self.expect_current(TokenKind::RightBrace)?;

            let end = self.current_token.span.end;

            return Ok(Statement::Function(Function {
                name: function_name,
                params,
                body,
                span: Span { start, end },
            }));
        }

        Err(format!("expected to close the block with a right brace."))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_token.span.start;
        self.next_token(); // consume return token

        let argument = self.parse_expression(Precedence::Lowest)?.0;

        if self.peek_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        let end = self.current_token.span.end;

        Ok(Statement::Return(Return {
            argument,
            span: Span { start, end },
        }))
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, ParseError> {
        let start = self.current_token.span.start;
        self.next_token();

        let mut block_statement: Vec<Statement> = Vec::new();

        while !self.current_token_is(TokenKind::RightBrace)
            && !self.current_token_is(TokenKind::EOF)
        {
            let statement = self.parse_statement()?;
            block_statement.push(statement);
            self.next_token();
        }

        let end = self.current_token.span.end;

        Ok(BlockStatement {
            body: block_statement,
            span: Span { start, end },
        })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParseError> {
        let start = self.current_token.span.start;

        // lets read the condition
        self.expect_peek(TokenKind::LeftParen)?;
        let (condition, _) = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(TokenKind::RightParen)?;

        // consequent stands for body of the if statement
        let consequent = Box::new(self.parse_block_statement()?);

        // TODO - implement else if statements

        let alternate: Option<Box<BlockStatement>> = if self.peek_token_is(TokenKind::Else) {
            self.next_token(); // consume else token

            self.expect_peek(TokenKind::LeftBrace)?;

            Some(Box::new(self.parse_block_statement()?))
        } else {
            None
        };

        let end = self.current_token.span.end;

        Ok(Statement::If(If {
            condition,
            consequent,
            alternate,
            span: Span { start, end },
        }))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.parse_expression(Precedence::Lowest)?.0;

        if self.peek_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        Ok(Statement::Expression(expr))
    }

    // Parse expressions
    fn parse_function_call_expression(&mut self, left: Expression, left_start: usize) -> Result<Expression, ParseError>{
        let arguments = self.parse_expression_series(TokenKind::RightParen)?;

        let end = self.current_token.span.end;

        Ok(Expression::FunctionCall(FunctionCall {
            call: Box::new(left),
            arguments: arguments.0,
            span: Span { start: left_start, end },
        }))
    } 

    fn parse_bool_expression(&mut self, token_kind: TokenKind) -> Result<Expression, ParseError> {
        let bool_literal = Expression::Literal(Literal::Boolean(Boolean {
            raw: token_kind == TokenKind::True,
            span: self.current_token.span.clone(),
        }));

        return Ok(bool_literal);
    }

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
            token_kind @ TokenKind::True | token_kind @ TokenKind::False => {
                return self.parse_bool_expression(token_kind.clone());
            }
            _ => {
                return Err(format!(
                    "no prefix function found for the token: {}",
                    self.current_token.kind
                ));
            }
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
            | TokenKind::GreaterThan => {
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
            
            TokenKind::LeftParen => {
                self.next_token(); // consume the identifier token
                return Some(self.parse_function_call_expression(left, left_start));
            }

            // TODO - Implement array index epxression parser
            _ => None,
        }
    }
}
