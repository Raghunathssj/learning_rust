#![allow(unused_variables)]
fn main() {
  // vector can only holds of same data types
  // declaring a vector with same type of numbers
  let v = vec![1,2,3,4];
  for x in v {
    println!("{:?}",x);
  }
  // declaring a vector with a initial value
  let v = vec![1;3];
  // can't use a vector if we use them with out reference (only in looping)
  // it can't be access at any cost even if you break the loop in middle (only in looping)
  for x in v {
    println!("{:?}",x);
  }
  // can access a vector like accessing an array
  // and a vector will only access using "usize" type

  //trying to access vector with an index more than its size
  let v = vec![1, 2, 3];
  // match is nothing but an switch in C lang
  match v.get(7) {
    Some(x) => println!("Item 7 is {}", x),
    None => println!("Sorry, this vector is too short.")
  }
  // you can also do push and pop in vectors if it is mutable
}
