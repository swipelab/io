use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::process::{exit};
use rune::io::eval::{eval, Context, RuntimeValue};
use rune::io::lexer::{tokenize};
use rune::io::parser::parse;


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
    println!("(ext) print > {:?}", args);
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
        let program = parse(tokens);
        let result = eval(program, ctx.clone());
        println!("> {:?}", result);
      }
    }
  }
}
