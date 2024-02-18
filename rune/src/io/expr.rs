#[derive(Debug, Clone)]
pub enum Expr {
  Program(Vec<Expr>),
  VarDecl { constant: bool, identifier: Symbol, value: Box<Expr> },
  FnDecl { identifier: Symbol, params: Vec<Parameter>, body: Box<Expr> },
  BinaryExpr { left: Box<Expr>, right: Box<Expr>, op: String },
  AssignExpr { target: Box<Expr>, value: Box<Expr> },
  Eq { left: Box<Expr>, right: Box<Expr> },
  NotEq { left: Box<Expr>, right: Box<Expr> },
  MemberExpr { object: Box<Expr>, property: Box<Expr>, computed: bool },
  CallExpr { caller: Box<Expr>, args: Vec<Expr> },
  IfExpr { when: Box<Expr>, then: Box<Expr>, other: Option<Box<Expr>> },
  Body { body: Vec<Expr> },
  Loop { body: Vec<Expr> },
  Break,
  Return { expr: Box<Expr> },

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

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
  pub name: String,
}

#[derive(Debug, Clone)]
pub struct Parameter {
  pub name: String,
}