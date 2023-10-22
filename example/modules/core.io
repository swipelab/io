module core;

// It never gets returned
type Never = [built_in];

// prints message and halts
panic : (message: Utf8) -> Never = [built_in];

panic<E: Debug> : (error: E) -> Never = {
  panic(e.format);
}

type Debug = trait{
  format: (self) -> Utf8;
}

type Result<R, E> = enum{
  Ok: R,
  Error: E,
}

impl<R,E> Result<R,E> {
  unwrap: (self) -> R where E: Debug = {
    match self {
      Ok(t) => t,
      Error(e) => panic("unwrap failed : {e.format()}"),
    }
  }
}
