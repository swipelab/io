package main

import (
	"bufio"
	"fmt"
	"math"
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

		ctx := eval.NewCtx()
		_, e := ctx.DeclareVar("pi", &eval.RuntimeFloat{Value: math.Pi})
		if e != nil {
			panic(e)
		}

		result := eval.Eval(&program, ctx).Marshal()
		fmt.Printf("> %s\n", result)
	}
}

func main() {
	repl()
}
