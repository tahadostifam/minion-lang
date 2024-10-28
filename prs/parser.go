package prs

import (
	"fmt"
	"log"

	"github.com/tahadostifam/minion-lang/ast"
	"github.com/tahadostifam/minion-lang/lexer"
	"github.com/tahadostifam/minion-lang/token"
)

type Parser struct {
	l *lexer.Lexer

	curToken  *token.Token
	peekToken *token.Token
}

func New(l *lexer.Lexer) *Parser {
	p := &Parser{l: l}

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
		fmt.Println(stmt)

		if stmt != nil {
			program.Statements = append(program.Statements, stmt)
		}

		p.nextToken()
	}

	fmt.Println(len(program.Statements))

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
	default:
		return nil
	}
}

func (p *Parser) expectPeek(t token.TokenType) bool {
	if p.isPeekToken(t) {
		p.nextToken()
		return true
	}

	return false
}

func (p *Parser) isCurToken(t token.TokenType) bool {
	return p.curToken.Type == t
}

func (p *Parser) isPeekToken(t token.TokenType) bool {
	return p.peekToken.Type == t
}

func (p *Parser) parseLetStatement() ast.Statement {
	log.Println(p.curToken)

	stmt := &ast.LetStatement{Token: p.curToken}

	if !p.expectPeek(token.IDENT) {
		return nil
	}

	stmt.Name = &ast.Identifier{Token: p.curToken, Literal: p.curToken.Literal}

	p.nextToken()

	if !p.expectPeek(token.ASSIGN) {
		return nil
	}

	for !p.isCurToken(token.SEMICOLON) {
		p.nextToken()
	}

	return stmt
}
