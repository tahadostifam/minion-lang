package main

import (
	"log"
	"os"

	"github.com/tahadostifam/minion-lang/lexer"
	prs "github.com/tahadostifam/minion-lang/prs"
)

func main() {
	if len(os.Args) == 1 {
		log.Fatalln("You must provide a code file path.")
	}

	filePath := os.Args[1]

	fileData, err := os.ReadFile(filePath)
	if err != nil {
		log.Fatalln(err)
	}

	l := lexer.New(string(fileData))
	p := prs.New(l)
	p.ParseProgram()
}
