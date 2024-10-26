package token

type TokenType string

type Token struct {
	Type    TokenType
	Literal string
}

var keywords = map[string]TokenType{
	"func": FUNCTION,
	"let":  LET,
}

func LookupIdent(ident string) TokenType {
	val, ok := keywords[ident]
	if ok {
		return val
	}

	return IDENT
}

const (
	ILLEGAL = "illegal"
	EOF     = "eof"

	// Identifiers + literals
	IDENT = "ident"
	INT   = "int"

	// Operators
	ASSIGN   = "="
	PLUS     = "+"
	MINUS    = "-"
	BANG     = "!"
	ASTERISK = "*"
	SLASH    = "/"
	LT       = "<"
	GT       = ">"
	EQ       = "=="
	NOT_EQ   = "!="

	// Delimiters
	COMMA     = ","
	SEMICOLON = ";"
	LPAREN    = "("
	RPAREN    = ")"
	LBRACE    = "{"
	RBRACE    = "}"

	// Keywords
	FUNCTION = "func"
	LET      = "let"
)

func NewToken(token TokenType, literal string) *Token {
	return &Token{token, literal}
}
