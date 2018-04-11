fn description() {
  // It is also same as "If-let" which is used for looping on a pattern matcher
  // It will loops till the pattern matches
}
fn main() {
  description();
  example();
}

// ####################################
fn example() {
  let mut v = vec![1, 3, 5, 7, 11];
  {
    loop {
      match v.pop() {
          Some(x) =>  println!("{}", x),
          None => break,
      }
    }
  }
  // The above block iterates through the vector and prints the values in vector
  // It can be do in a easier way with "while-let"
  while let Some(x) = v.pop() {
    println!("{}", x);
  }
}
