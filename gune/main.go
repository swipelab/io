package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
	"unicode"
)

type TokenKind string

const (
	TokenKind_Nil            TokenKind = "Nil"
	TokenKind_Number                   = "Number"
	TokenKind_Indentifier              = "Identifier"
	TokenKind_Let                      = "Let"
	TokenKind_BinaryOperator           = "BinaryOperator"
	TokenKind_Equals                   = "Equals"
	TokenKind_OpenParen                = "OpenParen"
	TokenKind_CloseParen               = "CloseParen"
	TokenKind_EOF                      = "EOF"
)

type NodeKind string

const (
	NodeKind_Program             NodeKind = "Program"
	NodeKind_Nil                          = "Nil"
	NodeKind_NumericLiteral               = "NumericLiteral"
	NodeKind_Identifier                   = "Identifier"
	NodeKind_BinaryExpression             = "BinaryExpression"
	NodeKind_FunctionDeclaration          = "FunctionDeclaration"
)

type RuntimeKind string

const (
	Runtime_Float RuntimeKind = "Float"
	Runtime_Nil               = "Nil"
)

var keywords = map[string]TokenKind{
	"let": TokenKind_Let,
	"nil": TokenKind_Nil,
}

var additiveOperators = []string{"-", "+"}
var multiplicativeOperators = []string{"*", "/", "&"}
var binaryOperators = append(additiveOperators, multiplicativeOperators...)

type RuntimeVal interface {
	Kind() RuntimeKind
}

type RuntimeFloat struct {
	//TODO: any number
	value float64
}

type RuntimeNil struct{}

func (e *RuntimeFloat) Kind() RuntimeKind {
	return Runtime_Float
}

func (e *RuntimeNil) Kind() RuntimeKind {
	return Runtime_Nil
}

type Token struct {
	kind  TokenKind
	value string
}

type Expression interface {
	kind() NodeKind
}

type Program struct {
	body []Expression
}

type BinaryExpression struct {
	left     Expression
	right    Expression
	operator string
}

type Identifier struct {
	symbol string
}

type NumericLiteral struct {
	value float64
}

type NilLiteral struct{}

func (e *Program) kind() NodeKind {
	return NodeKind_Program
}

func (e *BinaryExpression) kind() NodeKind {
	return NodeKind_BinaryExpression
}

func (e *Identifier) kind() NodeKind {
	return NodeKind_Identifier
}

func (e *NilLiteral) kind() NodeKind {
	return NodeKind_Nil
}

func (e *NumericLiteral) kind() NodeKind {
	return NodeKind_NumericLiteral
}

func isSkippable(e string) bool {
	r := e[0]
	return r == ' ' || r == '\n' || r == '\t'
}

func isAlpha(e string) bool {
	r := rune(e[0])
	return unicode.IsLetter(r)
}

func isInt(e string) bool {
	r := rune(e[0])
	return unicode.IsDigit(r)
}

func Tokenize(source string) []Token {
	tokens := make([]Token, 0)
	src := strings.Split(source, "")

	at := func() string {
		return src[0]
	}
	shift := func() string {
		val := src[0]
		src = src[1:]
		return val
	}
	push := func(kind TokenKind, value string) {
		tokens = append(tokens, Token{kind: kind, value: value})
	}

	for len(src) > 0 {
		if at() == "(" {
			push(TokenKind_OpenParen, shift())
		} else if at() == ")" {
			push(TokenKind_CloseParen, shift())
		} else if slices.Contains(binaryOperators, at()) {
			push(TokenKind_BinaryOperator, shift())
		} else if at() == "=" {
			push(TokenKind_Equals, shift())
		} else {

			if isInt(at()) {
				value := ""
				for len(src) > 0 && isInt(at()) {
					value += shift()
				}
				push(TokenKind_Number, value)
			} else if isAlpha(at()) {
				ident := ""
				for len(src) > 0 && (isInt(at()) && isAlpha(at())) {
					ident += shift()
				}

				reserved, ok := keywords[ident]
				if ok {
					push(reserved, ident)
				} else {
					push(TokenKind_Indentifier, ident)
				}
			} else if isSkippable(at()) {
				shift()
			} else {
				panic(fmt.Sprintf("Unknown char :%s", at()))
			}
		}
	}

	push(TokenKind_EOF, "EOF")
	return tokens
}

func BuildAST(source string) Program {
	tokens := Tokenize(source)
	program := Program{
		body: make([]Expression, 0),
	}

	at := func() Token {
		return tokens[0]
	}

	eat := func() Token {
		prev := tokens[0]
		tokens = tokens[1:]
		return prev
	}

	expect := func(kind TokenKind, err any) Token {
		prev := eat()
		if prev.kind != kind {
			panic(err)
		}
		return prev
	}

	eof := func() bool {
		return tokens[0].kind == TokenKind_EOF
	}

	parseExpression := func() Expression {
		panic(errors.New("Hoisting"))
	}

	parsePrimaryExpression := func() Expression {
		kind := at().kind
		switch kind {
		case TokenKind_Indentifier:
			return &Identifier{symbol: eat().value}
		case TokenKind_Nil:
			eat()
			return &NilLiteral{}
		case TokenKind_Number:
			value, _ := strconv.ParseFloat(eat().value, 64)
			return &NumericLiteral{
				value: value,
			}
		case TokenKind_OpenParen:
			{
				eat()
				expr := parseExpression()
				expect(TokenKind_CloseParen, errors.New("Ups... no closing paren"))
				return expr
			}
		default:
			panic(fmt.Sprintf("Ups %s", at()))
		}
	}

	parseMultiplicativeExpression := func() Expression {
		left := parsePrimaryExpression()
		for slices.Contains(multiplicativeOperators, at().value) {
			operator := eat().value
			right := parsePrimaryExpression()
			left = &BinaryExpression{
				left:     left,
				right:    right,
				operator: operator,
			}
		}
		return left
	}

	parseAdditiveExpression := func() Expression {
		left := parseMultiplicativeExpression()
		for slices.Contains(additiveOperators, at().value) {
			operator := eat().value
			right := parseMultiplicativeExpression()
			left = &BinaryExpression{
				left:     left,
				right:    right,
				operator: operator,
			}
		}
		return left
	}

	parseExpression = func() Expression {
		return parseAdditiveExpression()
	}

	parseStatement := func() Expression {
		//
		//
		return parseExpression()
	}

	for !eof() {
		program.body = append(program.body, parseStatement())
	}

	return program
}

func evalProgram(program Program) RuntimeVal {
	var lastEval RuntimeVal = &RuntimeNil{}
	for _, e := range program.body {
		lastEval = eval(e)
	}
	return lastEval
}

func evalNumericBinaryExpression(lhs *RuntimeFloat, rhs *RuntimeFloat, operator string) RuntimeVal {
	switch operator {
	case "+":
		return &RuntimeFloat{value: lhs.value + rhs.value}
	case "-":
		return &RuntimeFloat{value: lhs.value - rhs.value}
	case "*":
		return &RuntimeFloat{value: lhs.value * rhs.value}
	case "/":
		{
			//TODO: division by zero
			return &RuntimeFloat{value: lhs.value / rhs.value}
		}
	}
	panic("Ups")
}

func evalBinaryExpression(expr BinaryExpression) RuntimeVal {
	lhs := eval(expr.left)
	rhs := eval(expr.right)

	if lhs.Kind() == Runtime_Float || rhs.Kind() == Runtime_Float {
		return evalNumericBinaryExpression(lhs.(*RuntimeFloat), rhs.(*RuntimeFloat), expr.operator)
	}

	return &RuntimeNil{}
}

func eval(node Expression) RuntimeVal {
	switch node.kind() {
	case NodeKind_NumericLiteral:
		return &RuntimeFloat{value: node.(*NumericLiteral).value}
	case NodeKind_Nil:
		return &RuntimeNil{}
	case NodeKind_BinaryExpression:
		return evalBinaryExpression(*node.(*BinaryExpression))
	case NodeKind_Program:
		return evalProgram(*node.(*Program))
	default:
		panic(fmt.Sprintf("Ups AST %s", node))
	}
}

func repl() {
	fmt.Println("\nRepl IO v0.0.1")
	reader := bufio.NewReader(os.Stdin)
	for {
		input, _ := reader.ReadString('\n')
		input = strings.Replace(input, "\n", "", -1)
		program := BuildAST(input)
		result := eval(&program)
		fmt.Println(result)
	}
}

func main() {
	repl()
}
