package prs

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/tahadostifam/minion-lang/ast"
	"github.com/tahadostifam/minion-lang/lexer"
)

func TestParseLetStatement(t *testing.T) {
	testCases := []struct {
		input        string
		nameLiteral  string
		valueLiteral string
	}{
		{
			input:        `let foobar = 10;`,
			nameLiteral:  "foobar",
			valueLiteral: "10",
		},
		// {
		// 	input:        `let x = 500000;`,
		// 	nameLiteral:  "x",
		// 	valueLiteral: "500000",
		// },
		// {
		// 	input:        `let sample_variable = 0;`,
		// 	nameLiteral:  "sample_variable",
		// 	valueLiteral: "0",
		// },
	}

	for _, tc := range testCases {
		l := lexer.New(tc.input)
		p := New(l)

		program := p.ParseProgram()

		letStmt := program.Statements[0].(*ast.LetStatement)

		assert.Equal(t, letStmt.Name.Literal, tc.nameLiteral, "Let expression name literal does not match the specified value")

		// TODO - will work after implementing expression parser in parser.go
		// assert.Equal(t, letStmt.Value.TokenLiteral(), tc.valueLiteral, "Let expression value literal does not match the specified value")
	}
}

func TestParseLetStatementFailed(t *testing.T) {
	testCases := []struct {
		input string
	}{
		{
			input: `let = 10;`,
		},
		{
			input: `let 10;`,
		},
	}

	for _, tc := range testCases {

		l := lexer.New(tc.input)
		p := New(l)

		p.ParseProgram()

		t.Log("Input: ", tc.input)

		for _, v := range p.Errors() {
			t.Logf("Parser Error: %s", v)
		}

		t.Log("\n")
	}
}

func TestParseExpressions(t *testing.T) {
	testCases := []struct {
		input      string
		operator   string
		integerVal string
	}{
		{
			input:      "-10",
			operator:   "-",
			integerVal: "10",
		},
	}

	for _, tc := range testCases {
		l := lexer.New(tc.input)
		p := New(l)
		program := p.ParseProgram()

		assert.Len(t, p.Errors(), 0)
		assert.Len(t, program.Statements, 1)

		stmt, ok := program.Statements[0].(*ast.ExpressionStatement)
		assert.True(t, ok, "unable to cast program.Statements[0] to *ast.ExpressionStatement")

		expr, ok := stmt.Expression.(*ast.PrefixExpression)
		assert.True(t, ok, "unable to cast stmt.Expression to *ast.PrefixExpression")

		assert.Equal(t, expr.Operator, tc.operator)
		assert.Equal(t, expr.Right.TokenLiteral(), tc.integerVal)
	}
}