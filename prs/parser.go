package prs

import (
	"fmt"
	"strconv"

	"github.com/tahadostifam/minion-lang/ast"
	"github.com/tahadostifam/minion-lang/lexer"
	"github.com/tahadostifam/minion-lang/token"
)

type Parser struct {
	l *lexer.Lexer

	curToken  *token.Token
	peekToken *token.Token

	errors []string

	prefixParseFns map[token.TokenType]prefixParseFn
	infixParseFns  map[token.TokenType]infixParseFn
}

func New(l *lexer.Lexer) *Parser {
	p := &Parser{l: l, errors: []string{}, prefixParseFns: map[token.TokenType]prefixParseFn{}, infixParseFns: map[token.TokenType]infixParseFn{}}

	// we need to call nextToken twice to initialize the
	// curToken and peekToken correctly and then next calls
	// would happen by the ParseProgram automatically.
	p.nextToken()
	p.nextToken()

	p.registerPrefixFn(token.IDENT, p.parseIdentifier)
	p.registerPrefixFn(token.IDENT, p.parseIntegerLiteral)
	p.registerPrefixFn(token.PLUS, p.parsePrefixExpression)
	p.registerPrefixFn(token.MINUS, p.parsePrefixExpression)

	return p
}

func (p *Parser) parseIdentifier() ast.Expression {
	return &ast.Identifier{Token: p.curToken, Literal: p.curToken.Literal}
}

func (p *Parser) parseIntegerLiteral() ast.Expression {
	lit := &ast.IntegerLiteral{Token: p.curToken}

	val, err := strconv.ParseInt(p.curToken.Literal, 0, 64)
	if err != nil {
		p.errors = append(p.errors, fmt.Sprintf("could not parse %q as integer", p.curToken.Literal))
		return nil
	}

	lit.Value = val

	return lit
}

func (p *Parser) parsePrefixExpression() ast.Expression {
	expr := &ast.PrefixExpression{
		Token:    p.curToken,
		Operator: p.curToken.Literal,
	}

	p.nextToken()

	expr.Right = p.parseExpression(PREFIX)

	return expr
}

func (p *Parser) noPrefixParseFnError(t token.TokenType) {
	p.errors = append(p.errors, fmt.Sprintf("no prefix parse function found for %s", t))
}

func (p *Parser) ParseProgram() *ast.Program {
	program := &ast.Program{
		Statements: []ast.Statement{},
	}

	for p.curToken.Type != token.EOF {
		stmt := p.parseStatement()

		if stmt != nil {
			program.Statements = append(program.Statements, stmt)
		}

		p.nextToken()
	}

	return program
}

func (p *Parser) nextToken() {
	p.curToken = p.peekToken
	p.peekToken = p.l.NextToken()
}

func (p *Parser) parseStatement() ast.Statement {
	switch p.curToken.Type {
	case token.LET:
		return p.parseLetStatement()
	case token.RETURN:
		return p.parseReturnStatement()
	default:
		return p.parseExpressionStatement()
	}
}

func (p *Parser) parseExpression(precedence int) ast.Expression {
	prefix := p.prefixParseFns[p.curToken.Type]

	if prefix == nil {
		p.noPrefixParseFnError(p.curToken.Type)
		return nil
	}

	leftExp := prefix()

	return leftExp
}

func (p *Parser) parseExpressionStatement() *ast.ExpressionStatement {
	stmt := &ast.ExpressionStatement{Token: p.curToken}
	stmt.Expression = p.parseExpression(LOWEST)

	if p.isPeelToken(token.SEMICOLON) {
		p.nextToken()
	}

	return nil
}

func (p *Parser) parseReturnStatement() ast.Statement {
	stmt := &ast.ReturnStatement{Token: p.curToken}

	// TODO - parsing expression not implemented yet

	for !p.isCurToken(token.SEMICOLON) {
		p.nextToken()
	}

	return stmt
}

func (p *Parser) parseLetStatement() ast.Statement {
	stmt := &ast.LetStatement{Token: p.curToken}

	if !p.expectedToken(token.LET) {
		return nil
	}

	stmt.Name = &ast.Identifier{Token: p.curToken, Literal: p.curToken.Literal}
	p.nextToken()

	if !p.expectedToken(token.ASSIGN) {
		return nil
	}

	// TODO - parsing expression not implemented yet

	for !p.isCurToken(token.SEMICOLON) {
		p.nextToken()
	}

	return stmt
}

func (p *Parser) Errors() []string {
	return p.errors
}

func (p *Parser) isCurToken(t token.TokenType) bool {
	return p.curToken.Type == t
}

func (p *Parser) isPeelToken(t token.TokenType) bool {
	return p.peekToken.Type == t
}

func (p *Parser) expectedToken(t token.TokenType) bool {
	if p.curToken.Type == t {
		p.nextToken()
		return true
	} else {
		p.peekError(t)
		return false
	}
}

func (p *Parser) peekError(t token.TokenType) {
	msg := fmt.Sprintf("expected next token to be %v, but got %v", t, p.peekToken.Type)

	p.errors = append(p.errors, msg)
}

func (p *Parser) ParsedValidly() bool {
	return len(p.errors) == 0
}

func (p *Parser) peekPrecendence() int {
	if p, ok := precendences[p.peekToken.Type]; ok {
		return p
	}

	return LOWEST
}

func (p *Parser) curPrecendence() int {
	if p, ok := precendences[p.curToken.Type]; ok {
		return p
	}

	return LOWEST
}

func (p *Parser) registerInfixFn(t token.TokenType, fn infixParseFn) {
	p.infixParseFns[t] = fn
}

func (p *Parser) registerPrefixFn(t token.TokenType, fn prefixParseFn) {
	p.prefixParseFns[t] = fn
}
