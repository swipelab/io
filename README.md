# io

Welcome to `io` programming language

## grammar

Summary

- comments: `//, /*...*/`
- unit: `()`
- scope: `{ ... }`
- equality: `!=` , `==`
- inequality: `<`, `<=`, `=>`, `>`
- logical operators: `&&`, `||`, `^^`
- types: `i`, `u`, `f`, `bool`, `()`, `pack`, `enum`, `struct`, `trait`, `set`
- binary operators: `*`, `+`, `-`, `/`, `%`
- bit operators: `!`, `&`, `|`, `^`, `<<`, `>>`
- generic: `Vec<T>`, `Map<K,V>`, `Set<T>`

## declaration

| Examples |                                      |
|----------|--------------------------------------|
| variable | let foo: i32 = 42;                   |
| function | foo: (bar: i32) -> i32 = { bar * 2 } |

```io

// Set
// Enables creating an arbitrary type set
type Accepted: set{bool; int; Utf8}


// Pack
// Eg: Color 
// ---Alpha|-----Red|---Green|----Blue|
type Color: pack{
  //32 bit unsigned int across   
  value: u32 = [0..<32];
  a: u8 = [0..<8];
  //skip 8 take 8
  r: u8 = [8..<16];
  g: u8 = [16..<24];
  b: u8 = [24..<32];
  rgb: u24 = [8..<32];
};

//
// 1. color = Color{value: 0xFFFFFFFF}
// 2. color = Color{a:255, r:255, g:255, b:255}
// 3. color = Color{a:0xFF, rgb: 0xFFFFFF} 

// Structs
type Info: struct{  
  description: Utf8;
};

type FancyType: struct{
  thing: Things;
  // unammed struct, internally it would be considered FancyType_info
  info: struct{ a, b: i32 };
};

// Experimental ADT 
type Things: enum{
  Car;
  Truck;
  //type name for the struct would be Things_Bike_0
  Bike(struct{info: Info, quantity: u32});
  Paint(Color);
  //Things_Parts_0
  Parts(Vec<struct{part: Utf8, color: Color}>);
  Link(Map<i32, struct{}>);
};

// Function
// No Arguments and No Return
type Callback : ()
// Getter Of T
type Getter<T>: () -> T
// Setter Of T
type Setter<T>: (T) 
// Convert
type Convert<I,O>: (I) -> O
// Alias, type encoding
type VectorOfInt = Vec<i32>;

/*
f = FancyType(
  thing = Things.Paint(Color{value: 0xFF0000FF}),
  info = (a = 42, b = 0),
);
bike = Things.Bike{info: Info{description: "Bike"}, quantity: 42},
*/



groupBy<K,V>: (items: Vec<T>, key: (item: T) -> K) -> Map<K, Vec<V>> = {
  items.fold(Map(), (acc, item) {
    item_key = key(item);
    match acc.get(item_key) {
      Some(vec) => vec,              
      None => acc[item_key] = Vec(),
    }..push(item)
  })
}

sort<T: Ord>: (mut items: Vec<T>) {
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

quickSort<T: Ord>: (mut items: Vec<T>) = {
  partition: (mut items: Vec<T>, low, hight: i32) -> i32 = {
    pivot = items[high];
    i = low - 1;
    for j=low; j<= high -1; j++ {
      if items[j] < pivot {
        i+=1;
        items[i], items[j] = items[j], items[i];
      }
    }
    i + 1
  }

  sort: (mut items: Vec<T>, low: i32, high: i32) = {
    if low < high {
      pi = partition(items, low, high);
      quickSort(items, low, pi - 1);
      quickSort(items, pi + 1, high);
    }
  }

  sort(items, 0, items.length() - 1);
}



main : () = {
  // assignment
  bar = 42;

  foo: () -> i32 = {
    bar
  }

  io.cout << "hello " << foo().to_string() << "\n";

  i = Vec( 3, 5, 6, 7, 8, 1, 2)..sort();
  io.cout << "Sorted: " << i.to_string() << "\n";
}
```
