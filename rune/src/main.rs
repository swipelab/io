use std::collections::HashMap;
use std::io::{Write};
use std::sync::{Arc, Mutex};
use std::process::{exit};
use rune::io::eval::{eval};
use rune::io::lexer::{tokenize};
use rune::io::parser::{parse};
use rune::io::runtime::{Context, RuntimeValue};

fn main() {
  println!();
  println!("io.repl v.0.0.1");

  let mut context = Context {
    parent: None,
    variables: HashMap::new(),
  };
  context.let_variable("pi", RuntimeValue::Float(std::f64::consts::PI));
  context.let_variable("true", RuntimeValue::Bool(true));
  context.let_variable("false", RuntimeValue::Bool(false));
  context.let_variable("print", RuntimeValue::ExternFn(|args, _| {
    println!("[extern::print] > {:?}", args);
    RuntimeValue::Void
  }));
  context.let_variable("status", RuntimeValue::ExternFn(|_, ctx| {
    ctx.lock().unwrap().variables.iter().for_each(|e| println!("{:?}", e));
    RuntimeValue::Void
  }));
  let ctx = Arc::new(Mutex::new(context));

  loop {
    print!("$ ");
    std::io::stdout().flush().unwrap();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    match line.as_str() {
      "exit\n" => exit(0),
      e => {
        let source = e;
        let tokens = tokenize(source);
        let parsed = parse(tokens);
        match parsed {
          Ok(program) => {
            let result = eval(program, ctx.clone());
            println!("> {:?}", result);
          }
          Err(error) => {
            println!("{}", error.message);
          }
        }
      }
    }
  }
}
