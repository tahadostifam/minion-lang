package repl

import (
	"bufio"
	"fmt"
	"io"

	"github.com/tahadostifam/minion-lang/lexer"
	"github.com/tahadostifam/minion-lang/token"
)

const PROMPT = "> "

func Start(in io.Reader, out io.Writer) {
	scanner := bufio.NewScanner(in)

	for {
		fmt.Print(PROMPT)

		if !scanner.Scan() {
			return
		}

		line := scanner.Text()
		l := lexer.New(line)

		for {
			tok := l.NextToken()

			if tok.Type == token.EOF {
				break
			}

			fmt.Printf("%+v\n", tok)
		}
	}
}
