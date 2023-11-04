use std::clone::Clone;
use std::sync::Mutex;
use phf::phf_map;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
  Let,
  Const,
  Number,
  Identifier,
  BinaryOperator,
  Equals,
  Semicolon,
  Colon,
  OpenParenthesis,
  CloseParenthesis,
  Type,
  Struct,
  Union,
  Enum,
  EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub value: String,
}

pub static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
  "let"=> TokenKind::Let,
  "const"=> TokenKind::Const,
  "type"=> TokenKind::Type,
  "enum"=> TokenKind::Enum,
  "union" => TokenKind::Union,
  "struct" => TokenKind::Struct,
};


pub fn tokenize(source: &str) -> Vec<Token> {
  let src = source;
  let mut tokens: Vec<Token> = vec![];
  let index = Mutex::new(0);

  fn is_skippable(e: &str) -> bool {
    match e {
      " " | "\n" | "\t" => true,
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
      "-" | "+" | "*" | "/" | "%" => push(TokenKind::BinaryOperator, shift()),
      "=" => push(TokenKind::Equals, shift()),
      ":" => push(TokenKind::Colon, shift()),
      ";" => push(TokenKind::Semicolon, shift()),
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
        if let Some(e) = KEYWORDS.get(value.as_str()) {
          push(*e, value.as_str())
        } else {
          push(TokenKind::Identifier, value.as_str())
        }
      }
      e if is_skippable(e) => { shift(); }
      e => {
        println!("unexpected char :>{e}");
        break;
      }
    }
  }
  push(TokenKind::EOF, "EOF");
  return tokens;
}