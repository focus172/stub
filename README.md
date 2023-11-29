# Stub

A trait for creating the minimum valid type for a given type.

```rust
use stub::*;

#[derive(Stub)]
struct Callback {
    func: fn() -> bool,
    name: String
}

fn main() {
let example = Callback {
    name: String::from("test"),
    ..Stub::stub()
};
}
```

