use std::sync::Mutex;
use crate::io::ast::{Expr, Parameter, Property, Symbol};
use crate::io::lexer::{Token, TokenKind};

// Order Precedence
// assign
// object
// additive
// multiplicative
// call
// member
// primary

pub struct ParseError {
  pub message: String,
}

pub type ParseResult = Result<Expr, ParseError>;

struct ProgramParser {
  tokens: Vec<Token>,
  index: Mutex<usize>,
}

impl From<Expr> for ParseResult {
  fn from(value: Expr) -> Self {
    Ok(value)
  }
}

impl From<ParseError> for ParseResult {
  fn from(value: ParseError) -> Self {
    Err(value)
  }
}


pub fn parse(tokens: Vec<Token>) -> ParseResult {
  ProgramParser { tokens, index: Mutex::new(0) }.parse()
}

impl ProgramParser {
  fn parse(&self) -> ParseResult {
    let mut body = vec![];
    while self.more() {

      // ;;;;;; (valid trailing semicolons)
      if self.at().kind == TokenKind::Semicolon {
        self.eat();
        continue;
      }

      let statement = self.parse_statement()?;

      match statement {
        Expr::Error(_) | Expr::Never => {
          return statement.into();
        }
        _ => {}
      }

      body.push(statement)
    }
    Expr::Program(body).into()
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

  fn expect(&self, kind: TokenKind) -> Result<Token, ParseError> {
    let prev = self.eat();
    if prev.kind != kind {
      let error = ParseError { message: format!("expected {:?}, found {:?}", kind, prev.kind) };
      return Err(error);
    }
    Ok(prev)
  }

  fn more(&self) -> bool {
    self.at().kind != TokenKind::EOF
  }

  // (LET | CONST) IDENT = EXPR;
  fn parse_var_declaration(&self) -> ParseResult {
    let constant = self.eat().kind == TokenKind::Const;
    let identifier = self.expect(TokenKind::Identifier)?.value;

    // TODO: undefined var declaration :thinking:
    // if self.at().kind == TokenKind::Semicolon {
    //   self.eat()
    // }

    self.expect(TokenKind::Equals)?;
    let expr = self.parse_expr()?;
    self.expect(TokenKind::Semicolon)?;

    return Expr::VarDecl {
      constant,
      identifier: Symbol { name: identifier },
      value: Box::new(expr),
    }.into();
  }

  fn parse_loop(&self) -> ParseResult {
    self.eat();
    let body = self.parse_statement_body()?;
    Expr::Loop { body }.into()
  }

  fn parse_statement_body(&self) -> Result<Vec<Expr>, ParseError> {
    let mut body = Vec::new();
    self.expect(TokenKind::OpenBrace)?;
    while self.at().kind != TokenKind::EOF && self.at().kind != TokenKind::CloseBrace {
      body.push(self.parse_statement()?);
      if self.at().kind == TokenKind::Semicolon {
        self.eat();
      }
    }
    self.expect(TokenKind::CloseBrace)?;
    Ok(body)
  }

  fn parse_if_statement(&self) -> ParseResult {
    self.eat();
    let condition = self.parse_expr()?;
    let then = Expr::Body { body: self.parse_statement_body()? };

    let other = if self.at().kind == TokenKind::Else {
      self.eat();
      let token = self.at().kind;
      let expr = match token {
        TokenKind::If => self.parse_if_statement()?,
        TokenKind::OpenBrace => Expr::Body { body: self.parse_statement_body()? },
        _ => self.parse_statement()?
      };
      Some(Box::new(expr))
    } else {
      None
    };

    Expr::IfExpr {
      when: Box::new(condition),
      then: Box::new(then),
      other,
    }.into()
  }

  fn parse_fn_declaration(&self) -> ParseResult {
    self.eat();
    let identifier = Symbol { name: self.expect(TokenKind::Identifier)?.value };
    let args = self.parse_args()?;

    let mut params = Vec::new();
    for arg in args {
      match arg {
        Expr::Identifier(e) => { params.push(Parameter { name: e.name.clone() }); }
        _ => return ParseError { message: "Only [Identifier]s expected".to_string() }.into()
      }
    }

    let body = Box::new(Expr::Body { body: self.parse_statement_body()? });

    Expr::FnDecl {
      identifier,
      params,
      body,
    }.into()
  }

  fn parse_expr(&self) -> ParseResult {
    self.parse_assign_expr()
  }

  fn parse_conditional_expr(&self) -> ParseResult {
    //TODO: a == b == c => a == b && b == c
    //NOTE: maybe unwinding the parsed expr if conditional
    let left = self.parse_add_expr()?;
    match self.at().kind {
      TokenKind::Eq => {
        self.eat();
        let right = self.parse_add_expr()?;
        return Expr::Eq {
          left: Box::new(left),
          right: Box::new(right),
        }.into();
      }
      TokenKind::NotEq => {
        self.eat();
        let right = self.parse_add_expr()?;
        return Expr::NotEq {
          left: Box::new(left),
          right: Box::new(right),
        }.into();
      }
      _ => {}
    }
    left.into()
  }

  fn parse_assign_expr(&self) -> ParseResult {
    let mut target = self.parse_object_expr()?;
    if self.at().kind == TokenKind::Equals {
      self.eat();
      let rhs = self.parse_assign_expr();
      target = Expr::AssignExpr {
        target: Box::new(target),
        value: Box::new(rhs?),
      };
    }
    target.into()
  }

  //TODO: replace with [named|anonymous] [struct|enum|pack]{}
  fn parse_object_expr(&self) -> ParseResult {
    // { key = expr,  }

    if self.at().kind != TokenKind::OpenBrace {
      return self.parse_conditional_expr();
    }
    self.eat();
    let mut props = Vec::new();
    while self.at().kind != TokenKind::CloseBrace {
      let identifier = Symbol { name: self.expect(TokenKind::Identifier)?.value };

      if self.at().kind == TokenKind::Comma {
        self.eat();
        props.push(Property { identifier, value: None });
        continue;
      } else if self.at().kind == TokenKind::CloseBrace {
        props.push(Property { identifier, value: None });
        continue;
      }
      self.expect(TokenKind::Colon)?;
      let value = self.parse_expr();
      props.push(Property { identifier, value: Some(value?) })
    }

    self.expect(TokenKind::CloseBrace)?;

    Expr::Object { props }.into()
  }

  fn parse_statement(&self) -> ParseResult {
    match self.at().kind {
      TokenKind::Let | TokenKind::Const => self.parse_var_declaration(),
      TokenKind::Fn => self.parse_fn_declaration(),
      TokenKind::If => self.parse_if_statement(),
      TokenKind::Loop => self.parse_loop(),
      TokenKind::OpenBrace => Expr::Body { body: self.parse_statement_body()? }.into(),
      TokenKind::Break => {
        self.eat();
        Expr::Break.into()
      }
      TokenKind::Return => {
        self.eat();
        Expr::Return {
          expr: Box::new(self.parse_expr()?),
        }.into()
      }
      _ => self.parse_expr(),
    }
  }

  fn parse_primary_expr(&self) -> ParseResult {
    let current = self.at();
    match current.kind {
      TokenKind::Number => Expr::Number(self.eat().value.clone()).into(),
      TokenKind::String => Expr::String(self.eat().value.clone()).into(),
      TokenKind::Identifier => Expr::Identifier(Symbol { name: self.eat().value.clone() }).into(),
      TokenKind::OpenParenthesis => {
        self.eat();
        let expr = self.parse_expr();
        self.expect(TokenKind::CloseParenthesis)?;
        expr
      }
      _ => ParseError { message: format!("unknown {:?}", current) }.into()
    }
  }

  fn parse_mul_expr(&self) -> ParseResult {
    let mut left = self.parse_call_member_expr()?;
    loop {
      match self.at().value.as_str() {
        "*" | "/" | "%" => {
          let op = self.eat().value.clone();
          let right = self.parse_call_member_expr()?;
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
    left.into()
  }

  fn parse_call_member_expr(&self) -> ParseResult {
    let member = self.parse_member_expr()?;
    if self.at().kind == TokenKind::OpenParenthesis {
      return self.parse_call_expr(member).into();
    }
    member.into()
  }

  fn parse_call_expr(&self, caller: Expr) -> ParseResult {
    let mut call_expr = Expr::CallExpr {
      caller: Box::new(caller),
      args: self.parse_args()?,
    };

    if self.at().kind == TokenKind::OpenParenthesis {
      call_expr = self.parse_call_expr(call_expr)?;
    }
    call_expr.into()
  }

  fn parse_member_expr(&self) -> ParseResult {
    let mut object = self.parse_primary_expr()?;
    while self.at().kind == TokenKind::Dot || self.at().kind == TokenKind::OpenBracket {
      let operator = self.eat();
      let property: Expr;
      let computed: bool;
      if operator.kind == TokenKind::Dot {
        computed = false;
        // get identifier
        property = self.parse_primary_expr()?;
      } else {
        computed = true;
        property = self.parse_expr()?;
        self.expect(TokenKind::CloseBracket)?;
      }

      object = Expr::MemberExpr {
        computed,
        object: Box::new(object),
        property: Box::new(property),
      }
    }

    object.into()
  }


  fn parse_args(&self) -> Result<Vec<Expr>, ParseError> {
    self.expect(TokenKind::OpenParenthesis)?;
    let args = if self.at().kind == TokenKind::CloseParenthesis { Vec::new() } else { self.parse_args_list()? };
    self.expect(TokenKind::CloseParenthesis)?;
    Ok(args)
  }

  fn parse_args_list(&self) -> Result<Vec<Expr>, ParseError> {
    let mut args = vec!(self.parse_expr()?);
    while self.at().kind == TokenKind::Comma {
      self.eat();
      args.push(self.parse_expr()?)
    }
    Ok(args)
  }

  fn parse_add_expr(&self) -> ParseResult {
    let mut left = self.parse_mul_expr()?;
    loop {
      match self.at().value.as_str() {
        "+" | "-" => {
          let op = self.eat().value.clone();
          let right = self.parse_mul_expr()?;
          left = Expr::BinaryExpr {
            left: Box::new(left),
            right: Box::new(right),
            op,
          }
        }
        _ => break
      }
    }
    left.into()
  }
}