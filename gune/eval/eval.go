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

func evalProgram(program ast.Program) RuntimeVal {
	var lastEval RuntimeVal = &RuntimeNil{}
	for _, e := range program.Body {
		lastEval = Eval(e)
	}
	return lastEval
}

func evalFloatBinaryExpression(lhs *RuntimeFloat, rhs *RuntimeFloat, operator string) RuntimeVal {

	switch operator {
	case "+":
		return &RuntimeFloat{Value: lhs.Value + rhs.Value}
	case "-":
		return &RuntimeFloat{Value: lhs.Value - rhs.Value}
	case "*":
		return &RuntimeFloat{Value: lhs.Value * rhs.Value}
	case "/":
		{
			//TODO: division by zero
			return &RuntimeFloat{Value: lhs.Value / rhs.Value}
		}
	}
	panic("ups")
}

func evalBinaryExpression(expr ast.BinaryExpression) RuntimeVal {
	lhs := Eval(expr.Left)
	rhs := Eval(expr.Right)

	if lhs.Kind() == Runtime_Float || rhs.Kind() == Runtime_Float {
		return evalFloatBinaryExpression(lhs.(*RuntimeFloat), rhs.(*RuntimeFloat), expr.Operator)
	}

	return &RuntimeNil{}
}

func Eval(node ast.Expression) RuntimeVal {
	switch node.Kind() {
	case ast.NodeKind_NumericLiteral:
		return &RuntimeFloat{Value: node.(*ast.NumericLiteral).Value}
	case ast.NodeKind_Nil:
		return &RuntimeNil{}
	case ast.NodeKind_BinaryExpression:
		return evalBinaryExpression(*node.(*ast.BinaryExpression))
	case ast.NodeKind_Program:
		return evalProgram(*node.(*ast.Program))
	default:
		panic(fmt.Sprintf("Ups AST %s", node))
	}
}
