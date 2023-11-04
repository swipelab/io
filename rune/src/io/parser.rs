use std::sync::Mutex;
use crate::io::ast::{Expr, Symbol};
use crate::io::lexer::{Token, TokenKind};

struct ProgramParser {
  tokens: Vec<Token>,
  index: Mutex<usize>,
}

pub fn parse(tokens: Vec<Token>) -> Expr {
  ProgramParser { tokens, index: Mutex::new(0) }.parse()
}

impl ProgramParser {
  fn parse(&self) -> Expr {
    let mut body = vec![];
    while self.more() {

      // ;;;;;; (valid trailing semicolons)
      if self.at().kind == TokenKind::Semicolon {
        self.eat();
        continue;
      }

      body.push(self.parse_statement())
    }
    Expr::Program(body)
  }

  fn at(&self) -> Token {
    let i = self.index.lock().unwrap();
    self.tokens[*i].clone()
  }

  fn eat(&self) -> Token {
    let token = self.at();
    let mut i = self.index.lock().unwrap();
    *i += 1;
    token
  }

  fn expect(&self, kind: TokenKind) -> Token {
    let prev = self.eat();
    if prev.kind != kind {
      println!("expected {:?}, found {:?}", kind, prev.kind)
    }
    prev
  }

  fn more(&self) -> bool {
    self.at().kind != TokenKind::EOF
  }

  // (LET | CONST) IDENT = EXPR;
  fn parse_var_declaration(&self) -> Expr {
    let constant = self.eat().kind == TokenKind::Const;
    let identifier = self.expect(TokenKind::Identifier).value;

    // TODO: define only :thinking:
    // if self.at().kind == TokenKind::Semicolon {
    //   self.eat()
    // }

    self.expect(TokenKind::Equals);
    let expr = self.parse_expression();
    self.expect(TokenKind::Semicolon);

    return Expr::VarDecl {
      constant,
      identifier: Symbol { name: identifier },
      value: Box::new(expr),
    };
  }

  fn parse_expression(&self) -> Expr {
    self.parse_additive_expression()
  }

  fn parse_statement(&self) -> Expr {
    match self.at().kind {
      TokenKind::Let | TokenKind::Const => self.parse_var_declaration(),
      _ => self.parse_expression(),
    }
  }

  fn parse_primary_expression(&self) -> Expr {
    let kind = self.at().kind;
    match kind {
      TokenKind::Number => {
        Expr::Number(
          self.eat().value.clone()
        )
      }
      TokenKind::Identifier => {
        Expr::Identifier(Symbol { name: self.eat().value.clone() })
      }
      TokenKind::OpenParenthesis => {
        self.eat();
        let expr = self.parse_expression();
        self.expect(TokenKind::CloseParenthesis);
        expr
      }
      _ => Expr::Never
    }
  }

  fn parse_multiplicative_expression(&self) -> Expr {
    let mut left = self.parse_primary_expression();
    loop {
      match self.at().value.as_str() {
        "*" | "/" => {
          let op = self.eat().value.clone();
          let right = self.parse_primary_expression();
          let copy = left.clone();
          left = Expr::BinaryExpr {
            left: Box::new(copy),
            right: Box::new(right),
            op,
          }
        }
        _ => break
      }
    }
    return left;
  }

  fn parse_additive_expression(&self) -> Expr {
    let mut left = self.parse_multiplicative_expression();
    loop {
      match self.at().value.as_str() {
        "+" | "-" => {
          let op = self.eat().value.clone();
          let right = self.parse_multiplicative_expression();
          left = Expr::BinaryExpr {
            left: Box::new(left),
            right: Box::new(right),
            op,
          }
        }
        _ => break
      }
    }
    return left;
  }
}