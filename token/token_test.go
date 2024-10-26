package token

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLookupIdent(t *testing.T) {
	assert.Equal(t, string(LookupIdent("let")), LET)
	assert.Equal(t, string(LookupIdent("func")), FUNCTION)
	assert.Equal(t, string(LookupIdent("abcd")), IDENT)
}
