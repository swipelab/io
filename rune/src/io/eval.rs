use std::borrow::ToOwned;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::io::ast::{Expr, Parameter, Property, Symbol};
use crate::io::runtime::{Context, RefContext, RuntimeValue, Signal};

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

fn eval_object(props: Vec<Property>, ctx: RefContext) -> RuntimeValue {
  let mut map = HashMap::new();
  for prop in props {
    let value = match prop.value {
      None => ctx.lock().unwrap().get_variable(prop.identifier.name.as_str()).unwrap(),
      Some(expr) => eval(expr, ctx.clone()),
    };
    map.insert(prop.identifier.name.clone(), value);
  }
  RuntimeValue::Object(map)
}

fn eval_call(caller: Expr, args: Vec<Expr>, ctx: RefContext) -> RuntimeValue {
  let a = args.into_iter().map(|e| eval(e, ctx.clone())).collect();
  let f = eval(caller.clone(), ctx.clone());

  match f {
    RuntimeValue::ExternFn(delegate) => delegate(a, ctx.clone()),
    RuntimeValue::Fn { body, params, decl_ctx, .. } => {
      let mut context = Context {
        parent: Some(decl_ctx.clone()),
        variables: HashMap::new(),
      };

      for (i, param) in params.iter().enumerate() {
        match a.get(i) {
          Some(value) => { context.let_variable(param.name.as_str(), value.to_owned()); }
          None => { return RuntimeValue::Error(format!("invalid args")); }
        }
      }

      let context = Arc::new(Mutex::new(context));
      let result = eval(*body, context.clone());
      if let RuntimeValue::Signal(e) = result.clone() {
        return match e {
          Signal::Break => RuntimeValue::Never,
          Signal::Return(e) => *e
        };
      }
      result
    }
    _ => RuntimeValue::Error(format!("{:?} not a function ", caller))
  }
}

fn eval_body(body: Vec<Expr>, ctx: RefContext) -> RuntimeValue {
  let mut result = RuntimeValue::Never;
  for expr in body {
    let line = eval(expr, ctx.clone());
    if let RuntimeValue::Signal(_) = line.clone() {
      return line;
    }
    result = line;
  }
  result
}


fn eval_loop(body: Vec<Expr>, ctx: RefContext) -> RuntimeValue {
  let mut result = RuntimeValue::Never;

  loop {
    for expr in body.clone() {
      let line = eval(expr, ctx.clone());
      if let RuntimeValue::Signal(e) = line.clone() {
        return match e {
          Signal::Break => result,
          Signal::Return(_) => line,
        };
      }
      result = line
    }
  }
}

fn eval_not_eq(left: Expr, right: Expr, ctx: RefContext) -> RuntimeValue {
  let l = eval(left, ctx.clone());
  let r = eval(right, ctx);

  match l {
    RuntimeValue::Bool(lv) => {
      if let RuntimeValue::Bool(rv) = r {
        return RuntimeValue::Bool(lv != rv);
      }
    }
    RuntimeValue::Int(lv) => {
      if let RuntimeValue::Int(rv) = r {
        return RuntimeValue::Bool(lv != rv);
      }
    }

    RuntimeValue::Float(lv) => {
      if let RuntimeValue::Float(rv) = r {
        return RuntimeValue::Bool(lv != rv);
      }
    }
    _ => {}
  }

  RuntimeValue::Error("not the same types".to_string())
}

fn eval_eq(left: Expr, right: Expr, ctx: RefContext) -> RuntimeValue {
  let l = eval(left, ctx.clone());
  let r = eval(right, ctx);

  match l {
    RuntimeValue::Bool(lv) => {
      if let RuntimeValue::Bool(rv) = r {
        return RuntimeValue::Bool(lv == rv);
      }
    }
    RuntimeValue::Int(lv) => {
      if let RuntimeValue::Int(rv) = r {
        return RuntimeValue::Bool(lv == rv);
      }
    }

    RuntimeValue::Float(lv) => {
      if let RuntimeValue::Float(rv) = r {
        return RuntimeValue::Bool(lv == rv);
      }
    }
    _ => {}
  }

  RuntimeValue::Error("not the same types".to_string())
}

fn eval_if(when: Expr, then: Expr, other: Option<Box<Expr>>, ctx: RefContext) -> RuntimeValue {
  let condition = eval(when, ctx.clone());
  match condition {
    RuntimeValue::Bool(branch) => {
      if branch {
        eval(then, ctx)
      } else if let Some(e) = other {
        eval(*e, ctx)
      } else {
        RuntimeValue::Never
      }
    }
    _ => RuntimeValue::Error("invalid condition".to_string())
  }
}

fn eval_fn_decl(identifier: Symbol, params: Vec<Parameter>, body: Box<Expr>, ctx: RefContext) -> RuntimeValue {
  let function = RuntimeValue::Fn {
    identifier: identifier.clone(),
    params,
    body,
    decl_ctx: ctx.clone(),
  };
  // declare the function
  ctx.lock().unwrap().let_variable(identifier.name.as_str(), function.clone());
  return function;
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
    Expr::Object { props } => eval_object(props, ctx.clone()),
    Expr::CallExpr { caller, args } => eval_call(*caller, args, ctx.clone()),
    Expr::FnDecl { identifier, params, body } => eval_fn_decl(identifier, params, body, ctx.clone()),
    Expr::Body { body } => eval_body(body, ctx.clone()),
    Expr::IfExpr { when, then, other } => eval_if(*when, *then, other, ctx.clone()),
    Expr::Loop { body } => eval_loop(body, ctx.clone()),
    Expr::Break => RuntimeValue::Signal(Signal::Break),
    Expr::Return { expr } => RuntimeValue::Signal(Signal::Return(Box::new(eval(*expr, ctx.clone())))),
    Expr::Eq { left, right } => eval_eq(*left, *right, ctx.clone()),
    Expr::NotEq { left, right } => eval_not_eq(*left, *right, ctx.clone()),
    Expr::String(e) => RuntimeValue::String(e),
    _ => RuntimeValue::Error(format!("{:?} doesn't implement [eval]", node))
  }
}