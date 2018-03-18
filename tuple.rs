fn main() {
  let tuple: (i32,char,&str) = (2,'d',"hello");
  println!("tuple is same as array but in array all values are of same type but, in tuple it may varies");
  println!("{:?} is a tuple",tuple); //to print tuple :? is needed to format tuple
  // or you can print using their index
  println!("{}",tuple.0);
  println!("{}",tuple.1);
  println!("{}",tuple.2);
}
