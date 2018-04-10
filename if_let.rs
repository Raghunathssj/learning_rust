fn description() {
  // It is simply like if statement with a pattern matcher instead of condition
  // Basically an "If" works on a given condition
  // But "If let" is used to match a pattern with in the condition of an if statement
  // It is used to reduce overhead of certain pattern matches (Using "match" or "if" statements)
  // It helps to express intent well
}
fn main() {
  description();
  example();
}

// ##################################
fn example() {
  // Here we are trying to match a pattern using match
  let option = Some(5);
  fn foo(x: i32) {println!("{}",x);}
  {
    match option {
      // It will match the pattern and calls foo function
      Some(x) => { foo(x) },
      None => {},
    }
  }
  // You can either do it using "if" instead of "match"
  {
    if option.is_some() { //is_some is an inbuilt function in Some()
      foo(option.unwrap()); // unwrap is getting the value inside of Some that we assigned
    }
  }
  // The above both blocks are doing same instead doing it in that way Rust introduced a way to do it
  if let Some(value) = option {
    println!("Pattern matched");
    foo(value);
  };
  // It is quite explainable than the previous cases
  //It will match the pattern of Some(value) and option, if both are equal than it will call foo method will called
  // But what happens if pattern won't match
  if let Some(value) = None {
    println!("Won't print");
    foo(value);
  };
  // As i said it is like "If" it won't do anything if pattern won't matches
  // You can also use else case in if let
  if let Some(x) = None {
    foo(x);
  } else {
    println!("Pattern not matched");
  }
}
