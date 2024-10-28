package ast

import "go/token"

type Node interface {
	TokenLiteral() string // only usesd for debugging and testing
}

type Identifier struct {
	Token   token.Token
	Literal string
}
