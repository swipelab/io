package ast

import (
	"fmt"
	"slices"
	"unicode"
)

type TokenKind string

const (
	TokenKind_Nil            TokenKind = "Nil"
	TokenKind_Number         TokenKind = "Number"
	TokenKind_Indentifier    TokenKind = "Identifier"
	TokenKind_Let            TokenKind = "Let"
	TokenKind_BinaryOperator TokenKind = "BinaryOperator"
	TokenKind_Equals         TokenKind = "Equals"
	TokenKind_OpenParen      TokenKind = "OpenParen"
	TokenKind_CloseParen     TokenKind = "CloseParen"
	TokenKind_EOF            TokenKind = "EOF"
)

type Token struct {
	Kind  TokenKind
	Value string
}

var keywords = map[string]TokenKind{
	"let": TokenKind_Let,
	"nil": TokenKind_Nil,
}

var AdditiveOperators = []string{"-", "+"}
var MultiplicativeOperators = []string{"*", "/", "&"}
var BinaryOperators = append(AdditiveOperators, MultiplicativeOperators...)

func Tokenize(source string) []Token {

	isSkippable := func(e string) bool {
		r := e[0]
		return r == ' ' || r == '\n' || r == '\t'
	}

	isAlpha := func(e string) bool {
		r := rune(e[0])
		return unicode.IsLetter(r)
	}

	isInt := func(e string) bool {
		r := rune(e[0])
		return unicode.IsDigit(r)
	}

	tokens := make([]Token, 0)
	src := source

	at := func() string {
		return src[:1]
	}
	shift := func() string {
		val := src[:1]
		src = src[1:]
		return val
	}
	push := func(kind TokenKind, value string) {
		tokens = append(tokens, Token{Kind: kind, Value: value})
	}

	for len(src) > 0 {
		if at() == "(" {
			push(TokenKind_OpenParen, shift())
		} else if at() == ")" {
			push(TokenKind_CloseParen, shift())
		} else if slices.Contains(BinaryOperators, at()) {
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
				for len(src) > 0 && (isInt(at()) || isAlpha(at())) {
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
