use std::sync::Mutex;
use crate::io::ast::{Expr, Parameter, Property, Symbol};
use crate::io::ast::Expr::{CallExpr, MemberExp};
use crate::io::lexer::{Token, TokenKind};

// Order Precedence
// assign
// object
// additive
// multiplicative
// call
// member
// primary

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

  fn parse_fn_declaration(&self) -> Expr {
    self.eat();
    let identifier = Symbol { name: self.expect(TokenKind::Identifier).value };
    let args = self.parse_args();

    let mut params = Vec::new();
    for arg in args {
      match arg {
        Expr::Identifier(e) => { params.push(Parameter { name: e.name.clone() }); }
        _ => return Expr::Error("Only Expr::Identifier expected".to_string())
      }
    }

    self.expect(TokenKind::OpenBrace);
    let mut body = Vec::new();
    while self.at().kind != TokenKind::EOF && self.at().kind != TokenKind::CloseBrace {
      body.push(self.parse_expr());
      if self.at().kind == TokenKind::Semicolon {
        self.eat();
      }
    }
    self.expect(TokenKind::CloseBrace);

    return Expr::FnDecl {
      identifier,
      params,
      body,
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
      return self.parse_add_expr();
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
      TokenKind::Fn => self.parse_fn_declaration(),
      _ => self.parse_expr(),
    }
  }

  fn parse_primary_expr(&self) -> Expr {
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

  fn parse_mul_expr(&self) -> Expr {
    let mut left = self.parse_call_member_expr();
    loop {
      match self.at().value.as_str() {
        "*" | "/" | "%" => {
          let op = self.eat().value.clone();
          let right = self.parse_call_member_expr();
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

  fn parse_call_member_expr(&self) -> Expr {
    let member = self.parse_member_expr();
    if self.at().kind == TokenKind::OpenParenthesis {
      return self.parse_call_expr(member);
    }
    member
  }

  fn parse_call_expr(&self, caller: Expr) -> Expr {
    let mut call_expr = CallExpr {
      caller: Box::new(caller),
      args: self.parse_args(),
    };

    if self.at().kind == TokenKind::OpenParenthesis {
      call_expr = self.parse_call_expr(call_expr);
    }
    return call_expr;
  }

  fn parse_member_expr(&self) -> Expr {
    let mut object = self.parse_primary_expr();
    while self.at().kind == TokenKind::Dot || self.at().kind == TokenKind::OpenBracket {
      let operator = self.eat();
      let property: Expr;
      let computed: bool;
      if operator.kind == TokenKind::Dot {
        computed = false;
        // get identifier
        property = self.parse_primary_expr();
      } else {
        computed = true;
        property = self.parse_expr();
        self.expect(TokenKind::CloseBracket);
      }

      object = MemberExp {
        computed,
        object: Box::new(object),
        property: Box::new(property),
      }
    }

    object
  }


  fn parse_args(&self) -> Vec<Expr> {
    self.expect(TokenKind::OpenParenthesis);
    let args = if self.at().kind == TokenKind::CloseParenthesis { Vec::new() } else { self.parse_args_list() };
    self.expect(TokenKind::CloseParenthesis);
    args
  }

  fn parse_args_list(&self) -> Vec<Expr> {
    let mut args = vec!(self.parse_expr());
    while self.at().kind == TokenKind::Comma {
      self.eat();
      args.push(self.parse_expr())
    }
    args
  }

  fn parse_add_expr(&self) -> Expr {
    let mut left = self.parse_mul_expr();
    loop {
      match self.at().value.as_str() {
        "+" | "-" => {
          let op = self.eat().value.clone();
          let right = self.parse_mul_expr();
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