package ast

import "github.com/tahadostifam/minion-lang/token"

type Statement interface {
	Node
}

type LetStatement struct {
	Token token.Token
	Name  *Identifier
	Value Expression
}

func (ls *LetStatement) TokenLiteral() string {
	return ls.Token.Literal
}
