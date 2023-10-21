## grammar

- [ ] comments: `//, /*...*/`
- [ ] scope: `{...}`
- [ ] equality: `!=` , `==`
- [ ] logical operators: `&&`, `||`, `^^`
- [ ] scalar types: `i`, `u`, `f`, `bool`
- [x] binary operators: `*`, `+`, `-`, `/`
- [ ] generic: `Vec<T>`, `Map<K,V>`, `Set<T>`, ...
- [ ] complex: struct,

## declaration

| Examples |                                                   |
| -------- | ------------------------------------------------- |
| variable | foo: i32 = 42;                                    |
| function | foo: (bar: i32) -> i32 = {...}                    |

## functions

[function_name] : [function_type] = [implementation]

```io
// Types

// Union
// ---Alpha|-----Red|---Green|----Blue|
type Color = union[
  value: u32,
  :struct{ a: u8, r: u8, g: u8, b: u8, },
  :struct{  : u8, rgb: u24, },
];

// Alias
type WheelSizes : Vec<i32>;

// Struct
type Info : struct{  
  description: Utf8;
}

// Experimental ADT 
// using <u8> for descriminator
type Things : enum<u8>[ 
  Car           = 1,
  Truck         = 2,
  Bike          = 3,
  Color: Color  = 4,  
];

groupBy<K,V>: (items: Vec<T>, key: (item: T) -> K) -> Map<K, Vec<V>> = {
  items.fold(Map.new(), (collector, item) {
    item_key = key(item);
    match collector.get(item_key) {
      Some(vec) => vec,              
      None => collector[item_key] = Vec.new(),
    }..push(item)
  })
}

sort<T>: (items: Vec<T>) {
  for i=0; i < items.len(); i+=1 {
    max = i;
    for j=i+1; j < items.len(); j+=1 {
      if(items[max] < items[j]) {
        max = j;
      }
    }
    items[i], items[max] = items[max], items[i]
  }
}

foo: () -> i32 = {
  42
}

main : () = {
  io::cout << "hello " << foo().to_string() << "\n";
}
```
