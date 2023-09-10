package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/swipelab/io/gune/ast"
	"github.com/swipelab/io/gune/eval"
)

func repl() {
	fmt.Println()
	fmt.Println("io.repl v0.0.1")
	reader := bufio.NewReader(os.Stdin)
	for {
		fmt.Printf("# ")
		input, _ := reader.ReadString('\n')
		input = strings.Replace(input, "\n", "", -1)
		program := ast.BuildAST(input)
		result := eval.Eval(&program).Marshal()
		fmt.Printf("> %s\n", result)
	}
}

func main() {
	repl()
}
