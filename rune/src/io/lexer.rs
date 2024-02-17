use std::clone::Clone;
use std::sync::Mutex;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
  Let,
  Const,
  Number,
  Identifier,
  BinaryOperator,
  Equals,
  Eq,
  NotEq,
  Not,
  Semicolon,
  Colon,
  Comma,
  Dot,
  OpenParenthesis,
  CloseParenthesis,
  OpenBrace,
  CloseBrace,
  OpenBracket,
  CloseBracket,
  Type,
  Struct,
  Fn,
  Union,
  Enum,
  If,
  Else,
  Pub,
  Match,
  Loop,
  Break,
  Return,
  EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub value: String,
}

pub fn keyword(value: &str) -> Option<TokenKind> {
  match value {
    "let" => Some(TokenKind::Let),
    "const" => Some(TokenKind::Const),
    "type" => Some(TokenKind::Type),
    "enum" => Some(TokenKind::Enum),
    "union" => Some(TokenKind::Union),
    "struct" => Some(TokenKind::Struct),
    "fn" => Some(TokenKind::Fn),
    "if" => Some(TokenKind::If),
    "else" => Some(TokenKind::Else),
    "pub" => Some(TokenKind::Pub),
    "match" => Some(TokenKind::Match),
    "loop" => Some(TokenKind::Loop),
    "break" => Some(TokenKind::Break),
    "return" => Some(TokenKind::Return),
    _ => None
  }
}

pub fn tokenize(source: &str) -> Vec<Token> {
  let src = source;

  let mut tokens: Vec<Token> = vec![];
  let index = Mutex::new(0);

  fn is_skippable(e: &str) -> bool {
    match e {
      " " | "\n" | "\t" | "\r" => true,
      _ => false,
    }
  }

  fn is_alphabetic(e: &str) -> bool {
    e.chars().all(char::is_alphabetic)
  }

  fn is_identifier(e: &str) -> bool {
    e.chars().all(|e|
      match e {
        a if char::is_alphabetic(a) => true,
        '_' => true,
        _ => false
      }
    )
  }

  fn is_int(e: &str) -> bool {
    e.chars().all(|e| e.is_ascii_digit())
  }

  fn is_number(e: &str) -> bool {
    e.chars().all(|e|
      match e {
        '.' | '0'..='9' => true,
        _ => false
      })
  }

  let more = || -> bool {
    let i = index.lock().unwrap();
    return src.len() > *i;
  };

  let at = || -> &str {
    let i = index.lock().unwrap();
    let result = &src[*i..*i + 1];
    return result;
  };

  let shift = || -> &str {
    let mut i = index.lock().unwrap();
    let val = &src[*i..*i + 1];
    *i += 1;
    return val;
  };

  let mut push = |kind, value: &str| {
    tokens.push(Token { kind, value: value.to_owned() })
  };

  while more() {
    match at() {
      "(" => push(TokenKind::OpenParenthesis, shift()),
      ")" => push(TokenKind::CloseParenthesis, shift()),
      "{" => push(TokenKind::OpenBrace, shift()),
      "}" => push(TokenKind::CloseBrace, shift()),
      "[" => push(TokenKind::OpenBracket, shift()),
      "]" => push(TokenKind::CloseBracket, shift()),
      ":" => push(TokenKind::Colon, shift()),
      ";" => push(TokenKind::Semicolon, shift()),
      "," => push(TokenKind::Comma, shift()),
      "." => push(TokenKind::Dot, shift()),
      "-" | "+" | "*" | "/" | "%" => push(TokenKind::BinaryOperator, shift()),
      "!" => {
        shift();
        match at() {
          "=" => {
            shift();
            push(TokenKind::NotEq, "!=");
          }
          _ => push(TokenKind::Not, "!"),
        }
      }
      "=" => {
        shift();
        match at() {
          "=" => {
            shift();
            push(TokenKind::Eq, "==");
          }
          _ => push(TokenKind::Equals, "="),
        }
      }
      e if is_int(e) => {
        let mut value = "".to_owned();
        while more() && is_number(at()) {
          value.push_str(shift())
        }
        push(TokenKind::Number, value.as_str())
      }
      e if is_alphabetic(e) => {
        let mut value = "".to_owned();
        while more() && is_identifier(at()) {
          value.push_str(shift())
        }
        if let Some(e) = keyword(value.as_str()) {
          push(e, value.as_str())
        } else {
          push(TokenKind::Identifier, value.as_str())
        }
      }
      e if is_skippable(e) => { shift(); }
      e => {
        println!("unexpected char [{e}]");
        break;
      }
    }
  }
  push(TokenKind::EOF, "EOF");
  return tokens;
}