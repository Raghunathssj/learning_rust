#![allow(unused_variables)]
#![allow(dead_code)]

fn imp() {
  // patterns are common in rust and those are very powerful also.
}

fn main() {
  // Before reading this know about match in rust
  // As we know that in match "_" acts as any case
  let x = 1;
  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
  }

  // You can also assign a value in match which has scope only on match
  // It will handle every case
  let y = 10;
  match x {
    y => println!("x: {} y: {}", x, y),
  }
  println!("y: {}",y);

  // This will give an warning called "unrechable pattern" because both are handling every case
  // match x {
  //   y => println!("x: {} y: {}", x, y),
  //   _ => println!("anything")
  // }

  imp();
  multiple_patterns();
  destructuring_patterns();
  ignoring_bindings();
  ref_and_ref_mut();
  pattern_ranges();
  pattern_bindings();
  pattern_guards();
  patterns_mix_and_match();
}

// #################################
fn multiple_patterns() {
  let x = 1;
  let y = 2;
  // You can also match with "|"
  match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
  }
}

// ##################################
fn destructuring_patterns() {
  struct Point {
    x: i32,
    y: i32,
  }
  let origin = Point { x: 0, y: 0 };
  // you can also destructure a structure using pattern
  match origin {
    Point { x, y } => println!("({},{})", x, y)
  }
  // you can also assign value to another variable using ":"
  match origin {
    Point { x:f, y:s } => println!("({},{})", f, s)
  }
  // you can only take out one values in a struct
  match origin {
    Point { x,.. } => println!("{}", x)
  }
  // you can take values at any position of a struct
  match origin {
    Point { y,.. } => println!("{}", y)
  }
}

// ####################################
fn ignoring_bindings() {
  let some_value: Result<i32, &'static str> = Err("There was an error");
  // "_" using this you can ignore any type and value, it will be valid for every pattern that creates a binding
  match some_value {
    Ok(value) => println!("got a value: {}", value),
    Err(_) => println!("an error occurred"),
  }
  fn coordinate() -> (i32, i32, i32) {
    (1, 2, 3)
  }
  // Using "_" you can nefglet any value at any position like below
  // It will not bind value at that position
  let (x, _, z) = coordinate();
  println!("{:?} {:?}",x,z);
  //------------------------------//
  let tuple: (u32, String) = (5, String::from("five"));
  let (x, _s) = tuple;
  // println!("Tuple is: {:?}", tuple); // this line will give an error
  // because the string is moved

  let tuple = (5, String::from("five"));
  let (x, _) = tuple;
  println!("Tuple is: {:?}", tuple); // this line won't give error because the string is not moved only the u32 is copied
  let _ = String::from("  hello  ").trim(); // here the string is directly dropping without binding
}

// ################################
fn ref_and_ref_mut() {
  let x = 5;
  match x {
    ref r => println!("Got a reference to {}", r) // here the r is getting reference of pattern that is &i32 by ref you can also make an mutual reference using ref mut
  }
  let mut x = 8;
  match x {
    ref mut mr => println!("Got a mutable reference to {}", mr),
  }
}

// #################################
fn pattern_ranges() {
  let x = 3;
  match x {
    1 ... 5 => println!("one through five"), // this line will work for any value from 1 to 5
    _ => println!("anything"),
  }
  // similarly with chars
  let x = '🤪';
  match x {
    'a' ... 'j' => println!("early letter"),
    'k' ... 'z' => println!("late letter"),
    _ => println!("something else"),
  }
  //Mostly ranges are used for intigers and characters
}

// #######################################
fn pattern_bindings() {
  let x = 1;
  // you can bind a value using "@"
  match x {
    e @ 1 ... 5 => println!("got a range element {}", e),
    _ => println!("anything"),
  }
  // you can do like the below also
  struct Person {
    name: Option<String>,
  }
  // here we are binding the name to a in inner function
  let name = "Steve".to_string();
  let x: Option<Person> = Some(Person { name: Some(name) });
  match x {
    Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
    _ => {}
  }
  // you can also use "|" to bind a value
  // but you need to bind the value on both sides
  let x = 8;
  match x {
    e @ 1 ... 5 | e @ 8 ... 10 => println!("got a range element {}", e),
    _ => println!("anything"),
  }
}

// ###################################
fn pattern_guards() {
  let x = 5;
  let b = true;
  // you can guard using if
  match x{
    i if i > 5 =>println!("got bigger than 5"),
    _ => println!("got less than or equal to 5")
  }
  // you can use if in multiple patterns also
  match x {
    3 | 4 | 5 if b => println!("Ok"),
    _ => println!("Nope")
  }
  // if you are using if in multiple patterns it will apply on all patterns
}

// ##################################
fn patterns_mix_and_match() {
  // like this you can also mix multiple patterns and use
  // match x {
  //   Foo { x: Some(ref name), y: None } => //some code here
  // }
}
