```rust,ignore
fn main() {
  let array: [i32;4] = [-1,2,3,4];
  for x in array.iter() {
    println!("{}",x);
  }
  for x in &array {
    println!("{}",x);
  }
  let array1 = [0;2];
  for x in &array1 {
    println!("{:?}",x);
  }
}
```
