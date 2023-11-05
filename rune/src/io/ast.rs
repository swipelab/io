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
  BinaryExpr { left: Box<Expr>, right: Box<Expr>, op: String },
  AssignExpr { target: Box<Expr>, value: Box<Expr> },

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