package ast

import "github.com/tahadostifam/minion-lang/token"

type Node interface {
	TokenLiteral() string // only usesd for debugging and testing
}

type Identifier struct {
	Token   *token.Token
	Literal string
}

// TokenLiteral implements Expression.
func (i *Identifier) TokenLiteral() string {
	return i.Token.Literal
}

type IntegerLiteral struct {
	Token *token.Token
	Value int64
}

func (il *IntegerLiteral) TokenLiteral() string {
	return il.Token.Literal
}

type PrefixExpression struct {
	Token    *token.Token
	Operator string
	Right    Expression
}

func (il *PrefixExpression) TokenLiteral() string {
	return il.Token.Literal
}

type InfixExpression struct {
	Token    *token.Token
	Left     Expression
	Operator string
	Right    Expression
}

func (ie *InfixExpression) TokenLiteral() string {
	return ie.Token.Literal
}
