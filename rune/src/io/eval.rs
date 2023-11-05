use std::borrow::ToOwned;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::sync::{Arc, Mutex};
use crate::io::ast::{Expr, Symbol};

type RefContext = Arc<Mutex<Context>>;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
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

fn eval_assign_expr(target: Expr, value: Expr, ctx: RefContext) -> RuntimeValue {
  match target {
    Expr::Identifier(ident) => eval_var_decl(ident, value, ctx),
    _ => RuntimeValue::Error(format!("only Expr::Identifier supported, found {:?}", target))
  }
}

fn eval_var_decl(identifier: Symbol, value: Expr, ctx: RefContext) -> RuntimeValue {
  let rvalue = eval(value, ctx.clone());
  let mut context = ctx.lock().unwrap();
  context.let_variable(identifier.name.as_str(), rvalue.clone())
}

fn eval_identifier(identifier: Symbol, ctx: RefContext) -> RuntimeValue {
  let mut context = ctx.lock().unwrap();
  context
    .get_variable(identifier.name.as_str())
    .unwrap_or(RuntimeValue::Error(format!("undefined {:?}", identifier)))
}

fn eval_binary_expr(left: Expr, right: Expr, op: String, ctx: RefContext) -> RuntimeValue {
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
pub struct Context {
  pub parent: Option<RefContext>,
  pub variables: HashMap<String, RuntimeValue>,
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
    // let a = 5; // define
    // let a = "bubu"; // re-define
    // if let Some(_) = self.variables.get(name) {
    //   return RuntimeValue::Error(format!("{name:?} already defined"));
    // }
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

pub fn eval(node: Expr, ctx: RefContext) -> RuntimeValue {
  match node {
    Expr::Program(e) => eval_program(e, ctx.clone()),
    Expr::Never => RuntimeValue::Never,
    Expr::Error(e) => RuntimeValue::Error(e),
    Expr::Number(e) => eval_number(e),
    Expr::BinaryExpr { left, right, op } => eval_binary_expr(*left, *right, op, ctx.clone()),
    Expr::Identifier(e) => eval_identifier(e, ctx.clone()),
    Expr::VarDecl { value, identifier, .. } => eval_var_decl(identifier, *value, ctx.clone()),
    Expr::AssignExpr { target: lhs, value: rhs } => eval_assign_expr(*lhs, *rhs, ctx.clone()),
    _ => RuntimeValue::Error(format!("{:?} doesn't implement [eval]", node))
  }
}