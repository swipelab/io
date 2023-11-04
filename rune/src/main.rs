use std::borrow::ToOwned;
use std::collections::HashMap;
use std::io::Write;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::{Arc, Mutex};
use std::process::exit;

type RefContext = Arc<Mutex<Context>>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum TokenKind {
  Let,
  Number,
  Identifier,
  BinaryOperator,
  Equals,
  OpenParenthesis,
  CloseParenthesis,
  Type,
  Struct,
  Union,
  Enum,
  EOF,
}

#[derive(Debug, Clone)]
struct Token {
  kind: TokenKind,
  value: String,
}

#[derive(Debug, Clone)]
enum Expr {
  Never,
  Program(Vec<Expr>),
  Number(String),
  BinaryExpr { left: Box<Expr>, right: Box<Expr>, op: String },
  Identifier(String),
}

#[derive(Debug, Clone)]
enum RuntimeValue {
  Never,
  Bool(bool),
  Float(f64),
  Int(i64),
  Error(String),
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
      _ => Expr::Never
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

fn eval_program(body: Vec<Expr>, ctx: RefContext) -> RuntimeValue {
  let mut result = RuntimeValue::Never;
  for expr in body {
    result = eval(expr, ctx.clone());
  }
  return result;
}

fn eval_number(value: String) -> RuntimeValue {
  if value.chars().any(|e| e == '.') {
    RuntimeValue::Float(value.parse::<f64>().unwrap())
  } else {
    RuntimeValue::Int(value.parse::<i64>().unwrap())
  }
}

fn eval_ident(name: &str, ctx: RefContext) -> RuntimeValue {
  let mut context = ctx.lock().unwrap();
  context.get_variable(name).unwrap_or(RuntimeValue::Error(format!("{name} undefined")))
}

fn eval_binary_expression(left: Expr, right: Expr, op: String, ctx: RefContext) -> RuntimeValue {
  let lhs = eval(left, ctx.clone());
  let rhs = eval(right, ctx.clone());

  match lhs {
    RuntimeValue::Error(e) => RuntimeValue::Error(e),
    RuntimeValue::Float(l) => {
      match rhs {
        RuntimeValue::Float(r) => return RuntimeValue::Float(eval_number_binary_operation(l, r, op.as_str())),
        RuntimeValue::Int(r) => return RuntimeValue::Float(eval_number_binary_operation(l, r as f64, op.as_str())),
        RuntimeValue::Error(e) => RuntimeValue::Error(e),
        _ => RuntimeValue::Never,
      }
    }
    RuntimeValue::Int(l) => {
      match rhs {
        RuntimeValue::Float(r) => return RuntimeValue::Float(eval_number_binary_operation(l as f64, r, op.as_str())),
        RuntimeValue::Int(r) => return RuntimeValue::Int(eval_number_binary_operation(l, r, op.as_str())),
        RuntimeValue::Error(e) => RuntimeValue::Error(e),
        _ => RuntimeValue::Never,
      }
    }
    _ => RuntimeValue::Never,
  }
}

#[derive(Debug)]
struct Context {
  parent: Option<RefContext>,
  variables: HashMap<String, RuntimeValue>,
}

impl Context {
  pub fn get_variable(&mut self, name: &str) -> Option<RuntimeValue> {
    match self.variables.get(name) {
      None => {}
      Some(v) => return Some(v.clone()),
    }

    match self.parent.clone() {
      None => None,
      Some(e) => {
        return e.lock().unwrap().get_variable(name);
      }
    }
  }
  pub fn let_variable(&mut self, name: &str, value: RuntimeValue) {
    if let Some(_) = self.variables.get(name) {
      panic!("{name:?} already defined");
    }
    self.variables.insert(name.to_owned(), value);
  }

  pub fn get_variable_context(self, variable_name: &str) -> Option<Arc<Context>> {
    match self.variables.get(variable_name) {
      None => {}
      Some(_) => {
        return Some(Arc::new(self));
      }
    }

    let mut parent = self.parent;
    loop {
      match parent {
        None => break,
        Some(e) => {
          parent = Some(e);
        }
      }
    }

    None
  }
}

fn eval(node: Expr, ctx: RefContext) -> RuntimeValue {
  match node {
    Expr::Program(e) => eval_program(e, ctx.clone()),
    Expr::Never => RuntimeValue::Never,
    Expr::Number(e) => eval_number(e),
    Expr::BinaryExpr { left, right, op } => eval_binary_expression(*left, *right, op, ctx),
    Expr::Identifier(e) => eval_ident(e.as_str(), ctx.clone()),
  }
}

fn tokenize(source: String) -> Vec<Token> {
  let src = source;
  let mut tokens: Vec<Token> = vec![];
  let index = Mutex::new(0);

  let keywords = HashMap::from([
    ("let".to_owned(), TokenKind::Let),
    ("type".to_owned(), TokenKind::Type),
    ("enum".to_owned(), TokenKind::Enum),
    ("union".to_owned(), TokenKind::Union),
    ("struct".to_owned(), TokenKind::Struct),
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

  let mut context = Context {
    parent: None,
    variables: HashMap::new(),
  };
  context.let_variable("pi", RuntimeValue::Float(std::f64::consts::PI));
  context.let_variable("true", RuntimeValue::Bool(true));
  context.let_variable("false", RuntimeValue::Bool(false));
  let ctx = Arc::new(Mutex::new(context));

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
        let result = eval(program, ctx.clone());
        println!("> {:?}", result);
      }
    }
  }
}
