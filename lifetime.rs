#![allow(unused_variables)]
#![allow(dead_code)]
fn description() {
  // every function that takes a reference as input or gives an reference as output
  //even though you won't mention lifetimes for every function compiler
  //but for somethings like struct you need to mention other wise it will give error
  //there are static and normal lifetimes
}

struct Hello<'a> {
  hello: &'a str
}

fn main() {
  description();
  // lifetime 'static is static is always alive
  // you can not mention static lifetime in structs or any function declerations
  // you can only mention static inside a function
  let x: &'static str = "hello";
  // for passing references you need to mention lifetime that of your wish
  // you can mention more than one lifetime in one function
  let y = Hello {hello:"hello"};
  hello(&x);
}

fn hello (s: &str){
  let x: &'static i32 = &5;
  println!("hello");
}
