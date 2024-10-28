package prs

import (
	"testing"

	"github.com/tahadostifam/minion-lang/lexer"
)

func TestParseLetStatement(t *testing.T) {
	input := `let a = 10;`

	l := lexer.New(input)
	p := New(l)

	program := p.ParseProgram()
	t.Log(program.Statements)
}
