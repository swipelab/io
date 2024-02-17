package eval

import (
	"fmt"

	"github.com/swipelab/io/gune/ast"
)

type RuntimeKind string

const (
	Runtime_Float RuntimeKind = "Float"
	Runtime_Nil   RuntimeKind = "Nil"
)

type RuntimeVal interface {
	Kind() RuntimeKind
	Marshal() string
}

type RuntimeFloat struct {
	//TODO: any number
	Value float64
}

type RuntimeNil struct{}

func (e *RuntimeFloat) Kind() RuntimeKind {
	return Runtime_Float
}

func (e *RuntimeFloat) Marshal() string {
	return fmt.Sprintf("%v", e.Value)
}

func (e *RuntimeNil) Kind() RuntimeKind {
	return Runtime_Nil
}

func (e *RuntimeNil) Marshal() string {
	return "nil"
}

func evalProgram(program ast.Program, ctx Ctx) RuntimeVal {
	var lastEval RuntimeVal = &RuntimeNil{}
	for _, e := range program.Body {
		lastEval = Eval(e, ctx)
	}
	return lastEval
}

func evalFloatBinaryExpression(lhs *RuntimeFloat, rhs *RuntimeFloat, operator string, ctx Ctx) RuntimeVal {
	switch operator {
	case "+":
		return &RuntimeFloat{Value: lhs.Value + rhs.Value}
	case "-":
		return &RuntimeFloat{Value: lhs.Value - rhs.Value}
	case "*":
		return &RuntimeFloat{Value: lhs.Value * rhs.Value}
	case "/":
		{
			if rhs.Value == 0.0 {
				panic("division by zero")
			}

			return &RuntimeFloat{Value: lhs.Value / rhs.Value}
		}
	}
	panic("unkown float operator")
}

func evalBinaryExpression(expr ast.BinaryExpression, ctx Ctx) RuntimeVal {
	lhs := Eval(expr.Left, ctx)
	rhs := Eval(expr.Right, ctx)

	if lhs.Kind() == Runtime_Float || rhs.Kind() == Runtime_Float {
		return evalFloatBinaryExpression(lhs.(*RuntimeFloat), rhs.(*RuntimeFloat), expr.Operator, ctx)
	}

	return &RuntimeNil{}
}

func evalIdentifier(ident ast.Identifier, ctx Ctx) RuntimeVal {
	v, e := ctx.lookupVar(ident.Symbol)
	if e != nil {
		panic(e)
	}
	return v
}

func Eval(node ast.Expression, ctx Ctx) RuntimeVal {
	switch val := node.(type) {
	case *ast.NumericLiteral:
		return &RuntimeFloat{Value: val.Value}
	case *ast.NilLiteral:
		return &RuntimeNil{}
	case *ast.BinaryExpression:
		return evalBinaryExpression(*val, ctx)
	case *ast.Identifier:
		return evalIdentifier(*val, ctx)
	case *ast.Program:
		return evalProgram(*val, ctx)
	default:
		panic("ups")
	}
}
