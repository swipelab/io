use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use crate::io::ast::{Expr, Parameter, Symbol};

pub type RefContext = Arc<Mutex<Context>>;
pub type RefSignal = Arc<Sender<Signal>>;

pub type ExternFn = fn(args: Vec<RuntimeValue>, ctx: RefContext) -> RuntimeValue;

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

impl fmt::Display for RuntimeValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self.to_owned() {
      RuntimeValue::Never => write!(f, "never"),
      RuntimeValue::Bool(e) => write!(f, "{:?}", e),
      RuntimeValue::Float(e) => write!(f, "{:?}", e),
      RuntimeValue::Int(e) => write!(f, "{:?}", e),
      RuntimeValue::String(e) => write!(f, "{:?}", e),
      RuntimeValue::Object(_) => write!(f, "object"),
      RuntimeValue::Error(e) => write!(f, "{:?}", e),
      RuntimeValue::ExternFn(_) => write!(f, "external_fn"),
      RuntimeValue::Fn { identifier, .. } => write!(f, "{:?}", identifier),
      RuntimeValue::Signal(e) => write!(f, "signal::{:?}", e),
    }
  }
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