package prs

import (
	"github.com/tahadostifam/minion-lang/ast"
	"github.com/tahadostifam/minion-lang/token"
)

type (
	prefixParseFn func() ast.Expression
	infixParseFn  func(ast.Expression) ast.Expression
)

const (
	_ int = iota
	LOWEST
	EQUALS      // ==
	LESSGREATER // < , >
	SUM         // +
	PRODUCT     // *
	PREFIX      // -X , !X
	CALL        // myFunc()
)

var precendences = map[token.TokenType]int{
	token.EQ:     EQUALS,
	token.NOT_EQ: EQUALS,

	token.LT: LESSGREATER,
	token.GT: LESSGREATER,

	token.PLUS:  SUM,
	token.MINUS: SUM,

	token.SLASH:    PRODUCT,
	token.ASTERISK: PRODUCT,
}
