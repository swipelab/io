use std::sync::Mutex;
use crate::io::ast::{Expr, Property, Symbol};
use crate::io::lexer::{Token, TokenKind};

// Precedence
// additive_expression
// multiplicative_expression
// primary_expression

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

      let statement = self.parse_statement();

      match statement {
        Expr::Error(_) | Expr::Never => {
          return statement;
        }
        _ => {}
      }

      body.push(statement)
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
    let expr = self.parse_expr();
    self.expect(TokenKind::Semicolon);

    return Expr::VarDecl {
      constant,
      identifier: Symbol { name: identifier },
      value: Box::new(expr),
    };
  }

  fn parse_expr(&self) -> Expr {
    self.parse_assign_expr()
  }

  fn parse_assign_expr(&self) -> Expr {
    let mut target = self.parse_object_expr();
    if self.at().kind == TokenKind::Equals {
      self.eat();
      let rhs = self.parse_assign_expr();
      target = Expr::AssignExpr {
        target: Box::new(target),
        value: Box::new(rhs),
      };
    }
    target
  }

  fn parse_object_expr(&self) -> Expr {
    // { key = expr,  }

    if self.at().kind != TokenKind::OpenBrace {
      return self.parse_additive_expr();
    }
    self.eat();
    let mut props = Vec::new();
    while self.at().kind != TokenKind::CloseBrace {
      let identifier = Symbol { name: self.expect(TokenKind::Identifier).value };

      if self.at().kind == TokenKind::Comma {
        self.eat();
        props.push(Property { identifier, value: None });
        continue;
      } else if self.at().kind == TokenKind::CloseBrace {
        props.push(Property { identifier, value: None });
        continue;
      }
      self.expect(TokenKind::Colon);
      let value = self.parse_expr();
      props.push(Property { identifier, value: Some(value) })
    }

    self.expect(TokenKind::CloseBrace);

    Expr::Object { props }
  }

  fn parse_statement(&self) -> Expr {
    match self.at().kind {
      TokenKind::Let | TokenKind::Const => self.parse_var_declaration(),
      _ => self.parse_expr(),
    }
  }

  fn parse_primary_expression(&self) -> Expr {
    let current = self.at();
    match current.kind {
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
        let expr = self.parse_expr();
        self.expect(TokenKind::CloseParenthesis);
        expr
      }
      _ => Expr::Error(format!("unknown {:?}", current))
    }
  }

  fn parse_multiplicative_expression(&self) -> Expr {
    let mut left = self.parse_primary_expression();
    loop {
      match self.at().value.as_str() {
        "*" | "/" | "%" => {
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

  fn parse_additive_expr(&self) -> Expr {
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