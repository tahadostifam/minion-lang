package ast

import "github.com/tahadostifam/minion-lang/token"

type Expression interface {
	Node
}

type ExpressionStatement struct {
	Token *token.Token
	Expression
}

func (es *ExpressionStatement) TokenLiteral() string {
	return es.Token.Literal
}
