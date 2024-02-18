use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{Write};
use std::sync::{Arc, Mutex};
use std::process::{exit};
use rune::io::eval::{eval};
use rune::io::lexer::{tokenize};
use rune::io::parser::{parse};
use rune::io::runtime::{Context, RefContext, RuntimeValue};

struct Query {
  filename: String,
}

fn parse_query(args: &[String]) -> Option<Query> {
  if args.len() < 2 { return None; }
  let filename = args[1].clone();
  Some(Query {
    filename,
  })
}

fn evaluate(source: &str, ctx: RefContext) {
  let tokens = tokenize(source);
  let parsed = parse(tokens);
  match parsed {
    Ok(program) => {
      let result = eval(program, ctx);
      println!("> {:?}", result);
    }
    Err(error) => {
      println!("{}", error.message);
    }
  }
}

fn main() {
  println!();
  println!("io.repl v.0.0.1");

  let args: Vec<String> = env::args().collect();
  let query = parse_query(&args);


  let mut context = Context {
    parent: None,
    variables: HashMap::new(),
  };
  context.let_variable("pi", RuntimeValue::Float(std::f64::consts::PI));
  context.let_variable("true", RuntimeValue::Bool(true));
  context.let_variable("false", RuntimeValue::Bool(false));
  context.let_variable("println", RuntimeValue::ExternFn(|args, _| {
    println!("[extern::print] > {:?}", args);
    RuntimeValue::Never
  }));
  context.let_variable("status", RuntimeValue::ExternFn(|_, ctx| {
    ctx.lock().unwrap().variables.iter().for_each(|e| println!("{:?}", e));
    RuntimeValue::Never
  }));

  let ctx = Arc::new(Mutex::new(context));

  if let Some(e) = query {
    let contents = fs::read_to_string(e.filename).expect("unable to read filename");
    evaluate(&contents, ctx.clone());
  }

  loop {
    print!("$ ");
    std::io::stdout().flush().unwrap();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    match line.as_str() {
      "exit\n" => exit(0),
      e => evaluate(e, ctx.clone()),
    }
  }
}
