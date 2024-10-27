package main

import (
	"os"

	"github.com/tahadostifam/minion-lang/repl"
)

func main() {
	repl.Start(os.Stdin, os.Stdout)
}
