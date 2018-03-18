fn main() {
  let x = "hai";
  if x == "hello" {
    println!("yup! it is hello");
  } else {
    println!("Nope! it is hai");
  }
  let mut y = if true {
    10
  } else {
    5
  };
  println!("if condition is true value of y will be {:?}",y);

  y = if false {
    10
  } else {
    5
  };
  println!("if condition is false value of y will be {:?}",y);

  // can use " else if " if you have more cases.
}
