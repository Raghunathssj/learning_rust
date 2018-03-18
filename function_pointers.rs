fn addition(first: i32,second: i32) -> i32 {
  first+second
}

fn main() {
  let sum: fn(i32,i32) -> i32 = addition;
  let subtract = subtraction;
  println!("{:?}",sum(1,2));
  println!("{:?}",subtract(2,1));
}

fn subtraction(first: i32,second: i32) -> i32 {
  return first-second;
}
