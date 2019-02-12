#![allow(unused_variables)]
fn main() {
  // A vector is simply an extendable array (not limited capacity)
  // Declaring a vector
  let vec = Vec::new();
  // Initializing while declaring a vector
  // Vector can only holds of same data types
  // Initializing a vector with same type of numbers
  let v = vec![1,2,3,4];
  // Reading a vector
  for x in v {
    println!("{:?}",x);
  }
  // Declaring a vector with a initial value
  let v = vec![1;3];
  // Can't use a vector if we use them with out reference (only in looping)
  // It can't be access at any cost even if you break the loop in middle (only in looping)
  for x in v {
    println!("{:?}",x);
  }
  // Can access a vector like accessing an array
  // And a vector will only access using "usize" type

  //Trying to access vector with an index more than its size
  let v = vec![1, 2, 3];
  // Match is nothing but an switch in C lang
  match v.get(7) {
    Some(x) => println!("Item 7 is {}", x),
    None => println!("Sorry, this vector is too short.")
  }
  // You can also do push and pop in vectors if it is mutable
  let vec = Vec::new();
  vec.push(2);
  // Extending a vector
  vec.extend([1, 2, 3].iter().cloned());
  // Initializing a vector with a given value in the given capacity
  // Here 0 is value and 5 is capacity
  let vec = vec![0; 5];
  // Another way is
  let mut vec = Vec::with_capacity(5);
  vec.resize(5, 0);
  // A vector can be passed as an argument with read only access using "Slice"
  fn read_slice(slice: &[usize]) {
    // ...
  }
  let v = vec![0, 1];
  read_slice(&v);
  // Assigning a sliced vector to a variable
  let x : &[usize] = &v;
  // For inbuilt methods of vector refer https://doc.rust-lang.org/std/vec/struct.Vec.html#methods
}
