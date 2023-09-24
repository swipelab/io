use std::borrow::ToOwned;
use std::collections::HashMap;
use std::sync::{Mutex};
use std::process::exit;

enum TokenKind {
  Let,
  Nil,
  Number,
  Identifier,
  BinaryOperator,
  Equals,
  OpenParenthesis,
  CloseParenthesis,
}

struct Token {
  kind: TokenKind,
  value: String,
}

fn tokenize(source: String) -> Vec<Token> {
  let src = source;
  let mut tokens: Vec<Token> = vec![];
  let index = Mutex::new(0);

  let keywords = HashMap::from([
    ("let".to_owned(), TokenKind::Let),
    ("nil".to_owned(), TokenKind::Nil)
  ]);

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
      "-" | "+" | "*" | "/" | "&" | "|" | "^" => push(TokenKind::BinaryOperator, shift()),
      "=" => push(TokenKind::Equals, shift()),
      e if is_int(e) => {
        let mut value = "".to_owned();
        while more() && is_int(at()) {
          value.push_str(shift())
        }
        push(TokenKind::Number, value.as_str())
      }
      e if is_alphabetic(e) => {
        let mut value = "".to_owned();
        while more() && is_identifier(at()) {
          value.push_str(shift())
        }
        push(TokenKind::Identifier, value.as_str())
      }
      e if is_skippable(e) => { shift(); }
      e => {
        println!("unexpected char :>{e}");
        break;
      }
    }
  }

  return tokens;
}

fn main() {
  println!();
  println!("io.repl v.0.0.1");
  loop {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    match line.as_str() {
      "exit" => { exit(0); }
      e => {
        let tokens = tokenize(e.to_owned());
        let token_len = tokens.len();
        println!("{token_len}");
      }
    }
  }
}
