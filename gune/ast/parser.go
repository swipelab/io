package ast

import (
	"errors"
	"fmt"
	"slices"
	"strconv"
)

func BuildAST(source string) Program {
	tokens := Tokenize(source)
	program := Program{
		Body: make([]Expression, 0),
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
		if prev.Kind != kind {
			panic(err)
		}
		return prev
	}

	eof := func() bool {
		return tokens[0].Kind == TokenKind_EOF
	}

	parseExpression := func() Expression {
		panic(errors.New("Hoisting"))
	}

	parsePrimaryExpression := func() Expression {
		kind := at().Kind
		switch kind {
		case TokenKind_Indentifier:
			return &Identifier{Symbol: eat().Value}
		case TokenKind_Nil:
			eat()
			return &NilLiteral{}
		case TokenKind_Number:
			value, _ := strconv.ParseFloat(eat().Value, 64)
			return &NumericLiteral{
				Value: value,
			}
		case TokenKind_OpenParen:
			{
				eat()
				expr := parseExpression()
				expect(TokenKind_CloseParen, errors.New("ups... no closing paren"))
				return expr
			}
		default:
			panic(fmt.Sprintf("Ups %s", at()))
		}
	}

	parseMultiplicativeExpression := func() Expression {
		left := parsePrimaryExpression()
		for slices.Contains(MultiplicativeOperators, at().Value) {
			operator := eat().Value
			right := parsePrimaryExpression()
			left = &BinaryExpression{
				Left:     left,
				Right:    right,
				Operator: operator,
			}
		}
		return left
	}

	parseAdditiveExpression := func() Expression {
		left := parseMultiplicativeExpression()
		for slices.Contains(AdditiveOperators, at().Value) {
			operator := eat().Value
			right := parseMultiplicativeExpression()
			left = &BinaryExpression{
				Left:     left,
				Right:    right,
				Operator: operator,
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
		program.Body = append(program.Body, parseStatement())
	}

	return program
}
