## grammar

Summary
- comments: `//, /*...*/`
- scope: `{...}`
- equality: `!=` , `==`
- logical operators: `&&`, `||`, `^^`
- scalar types: `i`, `u`, `f`, `bool`, 
- binary operators: `*`, `+`, `-`, `/`
- generic: `Vec<T>`, `Map<K,V>`, `Set<T>`
- complex: `union`, `enum`, `struct`

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
  value: u32;
  :struct{ a: u8, r: u8, g: u8, b: u8, };
  :struct{ a: u8, rgb: u24, };
};
// 1. color = Color(value = 0xFFFFFFFF)
// 2. color = Color(a=255, r=255, g=255, b=255)
// 3. color = Color(a=0xFF, rgb= 0xFFFFFF) // first byte is zero (alpha)

// Alias
type VecI32 = Vec<i32>;

// Struct
type Info = struct{  
  description: Utf8;
};

// Experimental ADT 
type Things = enum{
  Car;
  Truck;
  Bike: struct{info: Info, quantity: u32};
  Paint: Color;
};

type FancyType = struct{
  thing: Things;
  // unammed struct
  info: struct{ a, b: i32};
};

/*
f = FancyType(
  // notice the ctor for Things.Paint is Color.new()
  thing = Things.Paint(value= 0xFF0000FF),
  info = (a = 42, b = 0),
);
*/

type Option<T> = enum{
  Some: T;
  None;
};

groupBy<K,V>: (items: Vec<T>, key: (item: T) -> K) -> Map<K, Vec<V>> = {
  items.fold(Map(), (acc, item) {
    item_key = key(item);
    match acc.get(item_key) {
      Some(vec) => vec,              
      None => acc[item_key] = Vec(),
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
