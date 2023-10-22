use std::borrow::ToOwned;
use std::collections::HashMap;
use std::io::Write;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::{LockResult, Mutex, RwLock};
use std::process::exit;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, Clone)]
struct Token {
  kind: TokenKind,
  value: String,
}

#[derive(Debug, Clone)]
enum Expr {
  Program(Vec<Expr>),
  Nil,
  Number(String),
  BinaryExpr { left: Box<Expr>, right: Box<Expr>, op: String },
  Identifier(String),
}

#[derive(Debug, Copy, Clone)]
enum Result {
  Nil,
  Float(f64),
  Int(i64),
}

fn parse(tokens: Vec<Token>) -> Expr {
  ProgramParser { tokens, index: Mutex::new(0) }.parse()
}


struct ProgramParser {
  tokens: Vec<Token>,
  index: Mutex<usize>,
}

impl ProgramParser {
  fn parse(&self) -> Expr {


    let mut body = vec![];
    while self.more() {
      body.push(self.parse_statement())
    }
    Expr::Program(body)
  }

  fn at(&self) -> Token {
    let i = self.index.lock().unwrap();
    self.tokens[*i].clone()
  }

  fn eat(&self) -> Token {
    let token = self.at();
    let mut i = self.index.lock().unwrap();
    *i += 1;
    token
  }

  fn expect(&self, kind: TokenKind) -> Token {
    let prev = self.eat();
    if prev.kind != kind {
      println!("unexpected {:?}", prev.kind)
    }
    prev
  }

  fn more(&self) -> bool {
    self.at().kind != TokenKind::EOF
  }

  fn parse_expression(&self) -> Expr {
    self.parse_additive_expression()
  }

  fn parse_statement(&self) -> Expr {
    self.parse_expression()
  }

  fn parse_primary_expression(&self) -> Expr {
    let kind = self.at().kind;
    match kind {
      TokenKind::Nil => {
        self.eat();
        Expr::Nil {}
      }
      TokenKind::Number => {
        Expr::Number(
          self.eat().value.clone()
        )
      }
      TokenKind::Identifier => {
        Expr::Identifier(self.eat().value.clone())
      }
      TokenKind::OpenParenthesis => {
        self.eat();
        let expr = self.parse_expression();
        self.expect(TokenKind::CloseParenthesis);
        expr
      }
      _ => Expr::Nil
    }
  }

  fn parse_multiplicative_expression(&self) -> Expr {
    let mut left = self.parse_primary_expression();
    loop {
      match self.at().value.as_str() {
        "*" | "/" => {
          let op = self.eat().value.clone();
          let right = self.parse_primary_expression();
          let copy = left.clone();
          left = Expr::BinaryExpr {
            left: Box::new(copy),
            right: Box::new(right),
            op,
          }
        }
        _ => break
      }
    }
    return left;
  }

  fn parse_additive_expression(&self) -> Expr {
    let mut left = self.parse_multiplicative_expression();
    loop {
      match self.at().value.as_str() {
        "+" | "-" => {
          let op = self.eat().value.clone();
          let right = self.parse_multiplicative_expression();
          left = Expr::BinaryExpr {
            left: Box::new(left),
            right: Box::new(right),
            op,
          }
        }
        _ => break
      }
    }
    return left;
  }
}

fn eval_number_binary_operation<T>(lhs: T, rhs: T, op: &str) -> T
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

fn eval_program(body: Vec<Expr>, ctx: Rc<Context>) -> Result {
  let mut result = Result::Nil;
  for expr in body {
    result = eval(expr, ctx.clone());
  }
  return result;
}

fn eval_number(value: String) -> Result {
  if value.chars().any(|e| e == '.') {
    Result::Float(value.parse::<f64>().unwrap())
  } else {
    Result::Int(value.parse::<i64>().unwrap())
  }
}

fn eval_binary_expression(left: Expr, right: Expr, op: String, ctx: Rc<Context>) -> Result {
  let lhs = eval(left, ctx.clone());
  let rhs = eval(right, ctx.clone());

  match lhs {
    Result::Nil => {}
    Result::Float(l) => {
      match rhs {
        Result::Nil => {}
        Result::Float(r) => return Result::Float(eval_number_binary_operation(l, r, op.as_str())),
        Result::Int(r) => return Result::Float(eval_number_binary_operation(l, r as f64, op.as_str())),
      }
    }
    Result::Int(l) => {
      match rhs {
        Result::Nil => {}
        Result::Float(r) => return Result::Float(eval_number_binary_operation(l as f64, r, op.as_str())),
        Result::Int(r) => return Result::Int(eval_number_binary_operation(l, r, op.as_str())),
      }
    }
  }
  Result::Nil
}

struct Context {
  parent: Option<Rc<Context>>,
  vars: HashMap<String, Result>,
}

fn eval(node: Expr, ctx: Rc<Context>) -> Result {
  match node {
    Expr::Program(e) => eval_program(e, ctx),
    Expr::Nil => Result::Nil,
    Expr::Number(e) => eval_number(e),
    Expr::BinaryExpr { left, right, op } => eval_binary_expression(*left, *right, op, ctx),
    Expr::Identifier(e) => *ctx.vars.get(e.as_str()).unwrap(),
  }
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
    print!("$ ");
    std::io::stdout().flush().unwrap();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    match line.as_str() {
      "exit" => { exit(0); }
      e => {
        let source = e.to_owned();
        let tokens = tokenize(source);
        let program = parse(tokens);
        let result = eval(
          program,
          Rc::new(Context {
            parent: None,
            vars: HashMap::from([
              ("pi".to_owned(), Result::Float(std::f64::consts::PI))
            ]),
          }));
        println!("> {:?}", result);
      }
    }
  }
}
