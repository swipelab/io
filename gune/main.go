package main

import (
	"bufio"
	"fmt"
	"github.com/swipelab/io/gune/ast"
	"github.com/swipelab/io/gune/eval"
	"math"
	"os"
)

func repl() {
	fmt.Println()
	fmt.Println("io.repl v0.0.1")
	reader := bufio.NewReader(os.Stdin)
	for {
		fmt.Printf("# ")
		input, _ := reader.ReadString('\n')
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

/*

//funcs.io
pub fn getContact(i32 contactId) -> i32 {
  return v1+v2;
}

pub fn makeFamily(i32 man, i31 woman, i15 dog) -> isize {
 /// crazy
}

import 'funcs.io' as funcs;

pub fn main() -> i32 {
  man = 42;
  family = 42.funcs.makeFamily(41, 40);
  print(a.funfoo())
}

*/
