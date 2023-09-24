use std::borrow::ToOwned;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::{Mutex};
use std::process::exit;

#[derive(Debug, PartialEq)]
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


trait Expression {
  fn eval(&self) -> RuntimeValue;
}

#[derive(Debug)]
enum RuntimeValue {
  Nil,
  Float(f64),
  Int(i64),
}

struct ProgramExpression {
  body: Vec<Box<dyn Expression>>,
}

struct BinaryExpression {
  left: Box<dyn Expression>,
  right: Box<dyn Expression>,
  operator: String,
}

struct IdentifierExpression {
  symbol: String,
}

struct NumericLiteral {
  value: String,
}

struct NilLiteral {}

impl Expression for NilLiteral {
  fn eval(&self) -> RuntimeValue {
    RuntimeValue::Nil
  }
}

impl Expression for NumericLiteral {
  fn eval(&self) -> RuntimeValue {
    if self.value.chars().any(|e| e == '.') {
      RuntimeValue::Float(self.value.parse::<f64>().unwrap())
    } else {
      RuntimeValue::Int(self.value.parse::<i64>().unwrap())
    }
  }
}

impl Expression for IdentifierExpression {
  fn eval(&self) -> RuntimeValue {
    panic!("not yet")
  }
}

impl Expression for ProgramExpression {
  fn eval(&self) -> RuntimeValue {
    let mut result = RuntimeValue::Nil;
    for expr in self.body.iter() {
      result = expr.eval()
    }
    result
  }
}

fn eval_binary_operation<T>(lhs: T, rhs: T, op: &str) -> T
  where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>
{
  match op {
    "+" => lhs + rhs,
    "-" => lhs - rhs,
    "*" => lhs * rhs,
    "/" => lhs / rhs,
    _ => panic!()
  }
}

impl Expression for BinaryExpression {
  fn eval(&self) -> RuntimeValue {
    let lhs = self.left.eval();
    let rhs = self.right.eval();

    match lhs {
      RuntimeValue::Nil => {}
      RuntimeValue::Float(lv) => {
        match rhs {
          RuntimeValue::Nil => {}
          RuntimeValue::Float(rv) => return RuntimeValue::Float(eval_binary_operation(lv, rv, self.operator.as_str())),
          RuntimeValue::Int(rv) => return RuntimeValue::Float(eval_binary_operation(lv, rv as f64, self.operator.as_str())),
        }
      }
      RuntimeValue::Int(lv) => {
        match rhs {
          RuntimeValue::Nil => {}
          RuntimeValue::Float(rv) => return RuntimeValue::Float(eval_binary_operation(lv as f64, rv, self.operator.as_str())),
          RuntimeValue::Int(rv) => return RuntimeValue::Int(eval_binary_operation(lv, rv, self.operator.as_str())),
        }
      }
    }
    RuntimeValue::Nil
  }
}

fn built_ast(tokens: Vec<Token>) -> Box<dyn Expression> {
  let mut body: Vec<Box<dyn Expression>> = vec![];

  let index = Mutex::new(0);

  let at = || -> &Token {
    let i = index.lock().unwrap();
    &tokens[*i]
  };

  let eat = || {
    let token = at();
    let mut i = index.lock().unwrap();
    *i += 1;
    token
  };

  let expect = |kind: TokenKind| -> &Token{
    let prev = eat();

    if prev.kind != kind {
      println!("unexpected {:?}", prev.kind)
    }
    prev
  };

  let more = || -> bool {
    at().kind != TokenKind::EOF
  };

  let mut parse_expression = || -> Box<dyn Expression> {
    panic!("oh no...")
  };

  let parse_primary_expression = || -> Box<dyn Expression> {
    match at().kind {
      TokenKind::Nil => {
        eat();
        Box::new(NilLiteral {})
      }
      TokenKind::Number => {
        Box::new(NumericLiteral {
          value: eat().value.clone()
        })
      }
      TokenKind::Identifier => {
        Box::new(IdentifierExpression {
          symbol: eat().value.clone()
        })
      }
      TokenKind::OpenParenthesis => {
        eat();
        let expr = parse_expression();
        expect(TokenKind::CloseParenthesis);
        expr
      }
      _ => { return Box::new(NilLiteral {}); }
    }
  };

  let parse_multiplicative_expression = || -> Box<dyn Expression> {
    let mut left = parse_primary_expression();
    match at().value.as_str() {
      "*" | "/" => {
        let operator = eat().value.clone();
        let right = parse_primary_expression();
        left = Box::new(BinaryExpression {
          left,
          right,
          operator,
        })
      }
      _ => {}
    }
    left
  };

  let parse_additive_expression = || -> Box<dyn Expression> {
    let mut left = parse_multiplicative_expression();
    match at().value.as_str() {
      "+" | "-" => {
        let operator = eat().value.clone();
        let right = parse_multiplicative_expression();
        left = Box::new(BinaryExpression {
          left,
          right,
          operator,
        })
      }
      _ => {}
    }
    left
  };

  let parse_expression = || -> Box<dyn Expression> {
    parse_additive_expression()
  };

  let parse_statement = || -> Box<dyn Expression> {
    parse_expression()
  };

  while more() {
    body.push(parse_statement())
  }

  return Box::new(ProgramExpression { body });
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

  fn is_number(e: &str) -> bool {
    e.chars().all(|e|
      match e {
        '.' | '0'..='9' => true,
        _ => false
      })
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
        while more() && is_number(at()) {
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
        let result = program.eval();
        println!("{:?}", result)
      }
    }
  }
}
