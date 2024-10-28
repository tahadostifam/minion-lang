package prs

import (
	"github.com/tahadostifam/minion-lang/ast"
	"github.com/tahadostifam/minion-lang/lexer"
	"github.com/tahadostifam/minion-lang/token"
)

type Parser struct {
	l *lexer.Lexer

	curToken  *token.Token
	peekToken *token.Token

	errors []string
}

func New(l *lexer.Lexer) *Parser {
	p := &Parser{l: l, errors: []string{}}

	// we need to call nextToken twice to initialize the
	// curToken and peekToken correctly and then next calls
	// would happen by the ParseProgram automatically.
	p.nextToken()
	p.nextToken()

	return p
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
	}

	return nil
}

func (p *Parser) isCurToken(t token.TokenType) bool {
	return p.curToken.Type == t
}

func (p *Parser) isPeekToken(t token.TokenType) bool {
	return p.peekToken.Type == t
}

func (p *Parser) parseLetStatement() ast.Statement {
	stmt := &ast.LetStatement{Token: p.curToken}

	if p.curToken.Type != token.LET {
		return nil
	}
	p.nextToken()

	stmt.Name = &ast.Identifier{Token: p.curToken, Literal: p.curToken.Literal}
	p.nextToken()

	if p.curToken.Type != token.ASSIGN {
		return nil
	}
	p.nextToken()

	// TODO - parsing expression not implemented yet

	for !p.isCurToken(token.SEMICOLON) {
		p.nextToken()
	}

	return stmt
}
