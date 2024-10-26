package lexer

import (
	"strings"

	"github.com/tahadostifam/minion-lang/token"
)

type Lexer struct {
	input string

	currentPos int // points to current char
	nextPos    int // points after current char

	ch byte
}

func New(input string) *Lexer {
	l := &Lexer{input: input}
	l.readChar()
	return l
}

func (l *Lexer) readChar() {
	if l.nextPos >= len(l.input) {
		l.ch = 0
	} else {
		l.ch = l.input[l.nextPos]
	}

	l.currentPos = l.nextPos

	l.nextPos++
}

func (l *Lexer) NextToken() *token.Token {
	var tok *token.Token

	l.skipWhitespace()
	chStr := string(l.ch)

	switch l.ch {
	case '=':
		tok = token.NewToken(token.ASSIGN, chStr)
	case '+':
		tok = token.NewToken(token.PLUS, chStr)
	case '-':
		tok = token.NewToken(token.MINUS, chStr)
	case '!':
		tok = token.NewToken(token.BANG, chStr)
	case '*':
		tok = token.NewToken(token.ASTERISK, chStr)
	case '/':
		tok = token.NewToken(token.SLASH, chStr)
	case '<':
		tok = token.NewToken(token.LT, chStr)
	case '>':
		tok = token.NewToken(token.GT, chStr)
	case ';':
		tok = token.NewToken(token.SEMICOLON, chStr)
	case ',':
		tok = token.NewToken(token.COMMA, chStr)
	case '(':
		tok = token.NewToken(token.LPAREN, chStr)
	case ')':
		tok = token.NewToken(token.RPAREN, chStr)
	case '{':
		tok = token.NewToken(token.RBRACE, chStr)
	case '}':
		tok = token.NewToken(token.LBRACE, chStr)
	case 0:
		tok = token.NewToken(token.EOF, "")
	default:
		if isLetter(l.ch) {
			tok = token.NewToken(token.IDENT, l.readIdentifier())
			return tok
		} else if isNumber(l.ch) {
			tok = token.NewToken(token.INT, l.readNumber())
			return tok
		} else {
			tok = token.NewToken(token.ILLEGAL, chStr)
		}
	}

	l.readChar()
	return tok
}

func (l *Lexer) skipWhitespace() {
	if l.ch == ' ' || l.ch == '\t' || l.ch == '\n' || l.ch == '\r' {
		l.readChar()
	}
}

func (l *Lexer) readIdentifier() string {
	startPos := l.currentPos

	for isLetter(l.ch) {
		l.readChar()
	}

	return l.input[startPos:l.currentPos]
}

func (l *Lexer) readNumber() string {
	startPos := l.currentPos

	for isNumber(l.ch) {
		l.readChar()
	}

	return l.input[startPos:l.currentPos]
}

func isLetter(ch byte) bool {
	return strings.ToLower(string(ch)) != strings.ToUpper(string(ch))
}

func isNumber(ch byte) bool {
	return (ch >= '0' && ch <= '9')
}
