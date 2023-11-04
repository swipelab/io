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
  Identifier(Symbol),

  //TODO: turn into RuntimeValue
  Number(String),

  //TODO: Halt / Panic / ....
  Never,
}

#[derive(Debug, Clone)]
pub struct Symbol {
  pub name: String,
}