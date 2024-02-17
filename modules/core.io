module core;

// Never
extern Never;

// prints message and halts
extern panic : (message: Utf8) -> Never;

type Debug : trait{
  format: (self) -> Utf8;
}

panic<E: Debug> : (error: E) -> Never = {
  panic(e.format());
}

type Result<R, E>: enum{
  Ok: R,
  Error: E,
}

type Maybe<T> = enum{
  Some: T;
  None;
};
