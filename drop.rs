#![allow(unused_variables)]

struct DropExample{
  name: &'static str,
}

impl Drop for DropExample {
  fn drop(&mut self){
    println!("{} dropped",self.name);
  }
}
fn description() {
  // Here we are implementing Drop trait for DropExample which needs drop function
  // drop function takes mutable reference as parameter
  // This function will call automatically when the variable that is declared using the strucyt that implemented Drop trait goes out of scope
  // You can call the function by user self directly(drop(variable))
  // Values are dropped in reverse order of declaration
  // It works on LIFO(Last In First Out)
  // It is useful to clean up any resources that associated with struct
}


fn main() {
  description();
  auto_dropping();
  manual_dropping();
  dropping_multiple();
}

// #####################################
fn auto_dropping() {
  let x = DropExample{name:"x automatically"};
  // Here we are not calling drop() but it will automatically calls because x will go out of scope in next line
}

// #####################################
fn manual_dropping() {
  let x = DropExample{name:"x manually"};
  // drop(x);
  // println!("{}",x.name); // This line will give error that variable is moved
  let y = DropExample{name:"y"};
  drop(&y);
  // If you are dropping manually by passing reference of variable it won't drop the variable it will be available till it goes out of scope
  // Same with mutable reference also
  println!("{}", y.name);
}

// #####################################
fn dropping_multiple() {
  let first = DropExample{name:"first"}; // This will drop second
  let second = DropExample{name:"second"}; // This will drop first
}
