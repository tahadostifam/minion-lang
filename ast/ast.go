package ast

import "github.com/tahadostifam/minion-lang/token"

type Node interface {
	TokenLiteral() string // only usesd for debugging and testing
}

type Identifier struct {
	Token   *token.Token
	Literal string
}
