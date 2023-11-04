use std::borrow::ToOwned;
use std::collections::HashMap;
use std::io::Write;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::sync::{Arc, Mutex};
use std::process::{exit};
use rune::io::ast::{Expr, Symbol};
use rune::io::lexer::{tokenize};
use rune::io::parser::parse;

type RefContext = Arc<Mutex<Context>>;

#[derive(Debug, Clone)]
enum RuntimeValue {
  Never,
  Bool(bool),
  Float(f64),
  Int(i64),
  Error(String),
}

fn eval_number_binary_operation<T>(lhs: T, rhs: T, op: &str) -> T
  where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Rem<Output=T>
{
  match op {
    "+" => lhs + rhs,
    "-" => lhs - rhs,
    "*" => lhs * rhs,
    "/" => lhs / rhs,
    "%" => lhs % rhs,
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

fn eval_var_decl(identifier: Symbol, value: Expr, ctx: RefContext) -> RuntimeValue {
  let rvalue = eval(value, ctx.clone());
  {
    let mut context = ctx.lock().unwrap();
    context.let_variable(identifier.name.as_str(), rvalue.clone());
  }
  rvalue
}

fn eval_ident(ident: Symbol, ctx: RefContext) -> RuntimeValue {
  let mut context = ctx.lock().unwrap();
  context.get_variable(ident.name.as_str()).unwrap_or(RuntimeValue::Error(format!("{:?} undefined", ident)))
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
  pub fn let_variable(&mut self, name: &str, value: RuntimeValue) -> RuntimeValue {
    if let Some(_) = self.variables.get(name) {
      panic!("{name:?} already defined");
    }
    self.variables.insert(name.to_owned(), value.clone());
    value
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
    Expr::Error(e) => RuntimeValue::Error(e),
    Expr::Number(e) => eval_number(e),
    Expr::BinaryExpr { left, right, op } => eval_binary_expression(*left, *right, op, ctx.clone()),
    Expr::Identifier(e) => eval_ident(e, ctx.clone()),
    Expr::VarDecl { value, identifier, .. } => eval_var_decl(identifier, *value, ctx.clone()),
  }
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
        let source = e;
        let tokens = tokenize(source);
        let program = parse(tokens);
        let result = eval(program, ctx.clone());
        println!("> {:?}", result);
      }
    }
  }
}
