package lexer

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/tahadostifam/monkey-lang/token"
)

func TestNextToken(t *testing.T) {
	testCases := []struct {
		input  string
		tokens []token.TokenType
	}{
		{
			input: "1 + 2",
			tokens: []token.TokenType{
				token.INT,
				token.PLUS,
				token.INT,
			},
		},
		{
			input: "123+ 456",
			tokens: []token.TokenType{
				token.INT,
				token.PLUS,
				token.INT,
			},
		},
		{
			input: "13 +45006",
			tokens: []token.TokenType{
				token.INT,
				token.PLUS,
				token.INT,
			},
		},
		{
			input: "0+0",
			tokens: []token.TokenType{
				token.INT,
				token.PLUS,
				token.INT,
			},
		},
		{
			input: "(1 + 2) / 3	- 2 * 100",
			tokens: []token.TokenType{
				token.LPAREN,
				token.INT,
				token.PLUS,
				token.INT,
				token.RPAREN,
				token.SLASH,
				token.INT,
				token.MINUS,
				token.INT,
				token.ASTERISK,
				token.INT,
			},
		},
	}

	for _, tc := range testCases {
		l := New(tc.input)

		var tok *token.Token = l.NextToken()

		i := 0
		for tok.Type != token.EOF {
			assert.Equal(t, tok.Type, tc.tokens[i])
			tok = l.NextToken()
			i++
		}
	}
}

func TestReadChar(t *testing.T) {
	input := "abc{}(); &"
	l := New(input)

	for i := 1; i < len(input); i++ {
		l.readChar()

		assert.Equal(t, l.ch, input[i])
		assert.Equal(t, l.currentPos, i)
		assert.Equal(t, l.nextPos, i+1)
	}
}

func TestReadIdentifier(t *testing.T) {
	input := "abcd"
	l := New(input)
	ident := l.readIdentifier()
	assert.Equal(t, ident, input)
}

func TestIsLetter(t *testing.T) {
	testCases := []struct {
		input    byte
		isLetter bool
	}{
		{
			input:    'a',
			isLetter: true,
		},
		{
			input:    '+',
			isLetter: false,
		},
		{
			input:    ';',
			isLetter: false,
		},
		{
			input:    '0',
			isLetter: false,
		},
		{
			input:    '!',
			isLetter: false,
		},
	}

	for _, v := range testCases {
		assert.Equal(t, isLetter(v.input), v.isLetter, fmt.Sprintf("The char %v is not a letter!", string(v.input)))
	}
}

func TestIsNumber(t *testing.T) {
	testCases := []struct {
		input    byte
		isNumber bool
	}{
		{
			input:    'a',
			isNumber: false,
		},
		{
			input:    '1',
			isNumber: true,
		},
		{
			input:    '9',
			isNumber: true,
		},
		{
			input:    '!',
			isNumber: false,
		},
		{
			input:    '+',
			isNumber: false,
		},
	}

	for _, v := range testCases {
		assert.Equal(t, isNumber(v.input), v.isNumber)
	}
}
