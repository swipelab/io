package ast

type NodeKind string

const (
	NodeKind_Program             NodeKind = "Program"
	NodeKind_Nil                 NodeKind = "Nil"
	NodeKind_NumericLiteral      NodeKind = "NumericLiteral"
	NodeKind_Identifier          NodeKind = "Identifier"
	NodeKind_BinaryExpression    NodeKind = "BinaryExpression"
	NodeKind_FunctionDeclaration NodeKind = "FunctionDeclaration"
)

type Program struct {
	Body []Expression
}

type BinaryExpression struct {
	Left     Expression
	Right    Expression
	Operator string
}

type Identifier struct {
	Symbol string
}

type NumericLiteral struct {
	Value float64
}

type NilLiteral struct{}

type Expression interface {
	Kind() NodeKind
}

func (e *Program) Kind() NodeKind {
	return NodeKind_Program
}

func (e *BinaryExpression) Kind() NodeKind {
	return NodeKind_BinaryExpression
}

func (e *Identifier) Kind() NodeKind {
	return NodeKind_Identifier
}

func (e *NilLiteral) Kind() NodeKind {
	return NodeKind_Nil
}

func (e *NumericLiteral) Kind() NodeKind {
	return NodeKind_NumericLiteral
}
