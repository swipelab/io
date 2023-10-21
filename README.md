## grammar

- [ ] comments: `//, /*...*/`
- [ ] scope: `{...}`
- [ ] equality: `!=` , `==`
- [ ] logical operators: `&&`, `||`, `^^`
- [ ] scalar types: `i`, `u`, `f`, `bool`, 
- [x] binary operators: `*`, `+`, `-`, `/`
- [ ] generic: `Vec<T>`, `Map<K,V>`, `Set<T>`
- [ ] complex: `union`, `enum`, `struct`

## declaration

| Examples |                                      |
| -------- | ------------------------------------ |
| variable | foo: i32 = 42;                       |
| function | foo: (bar: i32) -> i32 = { bar * 2 } |

## sample

```io
// Union
// ---Alpha|-----Red|---Green|----Blue|
type Color = union{
  value: u32,
  :struct{ a: u8, r: u8, g: u8, b: u8, },
  :struct{  : u8, rgb: u24, },
};

// Alias
type VecI32 = Vec<i32>;

// Struct
type Info = {  
  description: Utf8;
};

// Experimental ADT 
type Things = enum{
  Car,
  Truck,
  Bike: struct{info: Info, quantity: u32},
  Paint: Color,  
};

type Option<T> = enum{
  Some: T,
  None,
};

groupBy<K,V>: (items: Vec<T>, key: (item: T) -> K) -> Map<K, Vec<V>> = {
  items.fold(Map(), (collector, item) {
    item_key = key(item);
    match collector.get(item_key) {
      Some(vec) => vec,              
      None => collector[item_key] = Vec(),
    }..push(item)
  })
}

sort<T>: (mut items: Vec<T>) {
  for i = 0; i < items.len(); i+=1 {
    max = i;
    // find max
    for j = i+1; j < items.len(); j+=1 {
      if(items[max] < items[j]) {
        max = j;
      }
    }
    // swap
    items[i], items[max] = items[max], items[i]
  }
}

main : () = {
  bar = 42;

  foo: () -> i32 = {
    bar
  }

  io.cout << "hello " << foo().to_string() << "\n";

  i = Vec( 3, 5, 6, 7, 8, 1, 2)..sort();
  io.cout << "Sorted: " << i.to_string() << "\n";
}
```
