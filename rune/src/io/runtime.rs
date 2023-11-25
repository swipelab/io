use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::io::ast::{Expr, Parameter, Symbol};

pub type RefContext = Arc<Mutex<Context>>;
pub type ExternFn = fn(args: Vec<RuntimeValue>, ctx: RefContext) -> RuntimeValue;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
  Never,
  Void,
  Bool(bool),
  Float(f64),
  Int(i64),
  Object(HashMap<String, RuntimeValue>),
  Error(String),
  ExternFn(ExternFn),
  Fn { identifier: Symbol, params: Vec<Parameter>, body: Box<Expr>, decl_ctx: RefContext },
  Break,
}

#[derive(Debug)]
pub struct Context {
  pub parent: Option<RefContext>,
  pub variables: HashMap<String, RuntimeValue>,
}