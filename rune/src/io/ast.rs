pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
}

#[derive(Debug, Clone)]
pub enum Expr {
  Program(Vec<Expr>),
  VarDecl { constant: bool, identifier: Symbol, value: Box<Expr> },
  FnDecl { identifier: Symbol, params: Vec<Parameter>, body: Vec<Expr> },
  BinaryExpr { left: Box<Expr>, right: Box<Expr>, op: String },
  AssignExpr { target: Box<Expr>, value: Box<Expr> },
  MemberExp { object: Box<Expr>, property: Box<Expr>, computed: bool },
  CallExpr { caller: Box<Expr>, args: Vec<Expr> },

  //TODO: Halt / Panic / ....
  Error(String),
  Never,

  // literals
  Identifier(Symbol),
  //TODO: turn into RuntimeValue
  String(String),
  Number(String),
  Object { props: Vec<Property> },
  Property(Box<Property>),
}

#[derive(Debug, Clone)]
pub struct Property {
  pub identifier: Symbol,
  pub value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
  pub name: String,
}

#[derive(Debug, Clone)]
pub struct Parameter {
  pub name: String,
}