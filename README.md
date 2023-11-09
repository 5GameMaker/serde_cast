# serde_cast
> Works nicely with [`you_can`](https://crates.io/crates/you_can)!

1. Have structs `A` and `B`
2. Make sure they derive `Serialize` and `Deserialize` respectively
3. Cast!

```rust
use serde::{Serialize, Deserialize};
use serde_cast::serde_cast;

#[derive(Serialize)]
struct A {
    field: String,
}

#[derive(Deserialize)]
struct B {
    field: String,
}

let a = A { field: "This was a mistake".to_string() };
let b: B = serde_cast!(&a);

assert_eq!(a.field, b.field);
```
