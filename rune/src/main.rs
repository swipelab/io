use std::borrow::ToOwned;
use std::collections::HashMap;
use std::sync::{Mutex};
use std::process::exit;
use std::ptr::null;
use crate::TokenKind::Identifier;

enum TokenKind {
  Let,
  Nil,
  Number,
  Identifier,
  BinaryOperator,
  Equals,
  OpenParenthesis,
  CloseParenthesis,
  EOF,
}

struct Token {
  kind: TokenKind,
  value: String,
}


trait Expression {}

struct ProgramExpression {
  body: Vec<dyn Expression>,
}

struct BinaryExpression {
  left: dyn Expression,
  right: dyn Expression,
  operator: String,
}

struct IdentifierExpression {
  symbol: String,
}

struct NumericLiteral {
  value: String,
}

struct NilLiteral {}

fn built_ast(tokens: Vec<Token>) -> ProgramExpression {
  let body: Vec<dyn Expression> = vec![];

  let index = Mutex::new(0);

  let at = || -> &Token {
    let i = index.lock().unwrap();
    &tokens[*i]
  };

  let eat = || {
    let token = at();
    let i = index.lock().unwrap();
    *i += 1;
    token
  };

  let expect = |kind: TokenKind| -> &Token{
    let prev = eat();

    if &prev.kind != kind {
      println!("unexpected {}", &prev.kind)
    }
    prev
  };

  let more = || -> bool {
    at().kind != TokenKind::EOF
  };

  let mut parse_expression = || -> dyn Expression {};
  let parse_primary_expression = || -> dyn Expression {
    match at().kind {
      TokenKind::Nil => {
        eat();
        NilLiteral {}
      }
      TokenKind::Number => {
        NumericLiteral {
          value: eat().value.clone()
        }
      }
      TokenKind::Identifier => {
        IdentifierExpression {
          symbol: eat().value.clone()
        };
      }
      TokenKind::OpenParenthesis => {
        eat();
        let expr = parse_expression();
        expect(TokenKind::CloseParenthesis);
        expr
      }
      _ => { return null(); }
    }
  };

  while more() {}

  return ProgramExpression { body };
}

fn tokenize(source: String) -> Vec<Token> {
  let src = source;
  let mut tokens: Vec<Token> = vec![];
  let index = Mutex::new(0);

  let keywords = HashMap::from([
    ("let".to_owned(), TokenKind::Let),
    ("nil".to_owned(), TokenKind::Nil)
  ]);

  fn is_skippable(e: &str) -> bool {
    match e {
      " " | "\n" | "\t" => true,
      _ => false,
    }
  }

  fn is_alphabetic(e: &str) -> bool {
    e.chars().all(char::is_alphabetic)
  }

  fn is_identifier(e: &str) -> bool {
    e.chars().all(|e|
      match e {
        a if char::is_alphabetic(a) => true,
        '_' => true,
        _ => false
      }
    )
  }

  fn is_int(e: &str) -> bool {
    e.chars().all(|e| e.is_ascii_digit())
  }

  let more = || -> bool {
    let i = index.lock().unwrap();
    return src.len() > *i;
  };

  let at = || -> &str {
    let i = index.lock().unwrap();
    let result = &src[*i..*i + 1];
    return result;
  };

  let shift = || -> &str {
    let mut i = index.lock().unwrap();
    let val = &src[*i..*i + 1];
    *i += 1;
    return val;
  };

  let mut push = |kind, value: &str| {
    tokens.push(Token { kind, value: value.to_owned() })
  };

  while more() {
    match at() {
      "(" => push(TokenKind::OpenParenthesis, shift()),
      ")" => push(TokenKind::CloseParenthesis, shift()),
      "-" | "+" | "*" | "/" | "&" | "|" | "^" => push(TokenKind::BinaryOperator, shift()),
      "=" => push(TokenKind::Equals, shift()),
      e if is_int(e) => {
        let mut value = "".to_owned();
        while more() && is_int(at()) {
          value.push_str(shift())
        }
        push(TokenKind::Number, value.as_str())
      }
      e if is_alphabetic(e) => {
        let mut value = "".to_owned();
        while more() && is_identifier(at()) {
          value.push_str(shift())
        }
        push(TokenKind::Identifier, value.as_str())
      }
      e if is_skippable(e) => { shift(); }
      e => {
        println!("unexpected char :>{e}");
        break;
      }
    }
  }
  push(TokenKind::EOF, "EOF");
  return tokens;
}

fn main() {
  println!();
  println!("io.repl v.0.0.1");
  loop {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    match line.as_str() {
      "exit" => { exit(0); }
      e => {
        let tokens = tokenize(e.to_owned());
        let program = built_ast(tokens);
      }
    }
  }
}
