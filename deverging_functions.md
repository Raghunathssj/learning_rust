```rust
#![allow(unused_variables)]

fn diverges() -> ! {
  panic!("This function never returns!");
}

fn main() {
  diverges();
}
```
