use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use crate::io::ast::{Expr, Parameter, Symbol};

pub type RefContext = Arc<Mutex<Context>>;
pub type RefSignal = Arc<Sender<Signal>>;

pub type ExternFn = fn(args: Vec<RuntimeValue>, ctx: RefContext) -> RuntimeValue;

enum Color {
  V1 { a: i32 },
  V2 { b: i64 },
}

#[derive(Debug, Clone)]
pub enum RuntimeValue {
  Never,
  Bool(bool),
  Float(f64),
  Int(i64),
  String(String),
  Object(HashMap<String, RuntimeValue>),
  Error(String),
  ExternFn(ExternFn),
  Fn { identifier: Symbol, params: Vec<Parameter>, body: Box<Expr>, decl_ctx: RefContext },

  /// investigate if we can move one level higher ( potentially at the state machine level )
  Signal(Signal),
}

//TODO: finalise idea --- dispatch break & return
#[derive(Debug, Clone)]
pub enum Signal {
  //Exit,
  Break,
  Return(Box<RuntimeValue>),
}

#[derive(Debug)]
pub struct Context {
  pub parent: Option<RefContext>,
  pub variables: HashMap<String, RuntimeValue>,
}