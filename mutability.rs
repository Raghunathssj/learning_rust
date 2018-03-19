fn description() {
  //In normal if you declare a variable you can not its value
  // but using mut (mutability) you can change the value
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
  description();
  let x = "not mutable";
  // x = "won't change"; //this line will give you error because you are changing immutable variable
  let mut y = "mutable";
  y = "changed"; // this will change because of mutable of variable
  println!("{:?}",y);
  referncing_mut();
  reassigning_reference();
  mutating_immutable_variable();
  interior_mutability();
  mutating_struct();
}

fn referncing_mut() {
  // you can also assign mutable refernce to other variable
  let mut x = 5;
  // you can't assign the y variable refernce to another variable
  let y = &mut x;
  // but you can assign value to x using y
  *y = 6;
  println!("{}",y);
}

#[allow(unused_variables)]
fn reassigning_reference(){
  // but you can assign mutable refernce of y like this
  let mut x = 5;
  let mut y = &mut x;
  let z = &mut y;
}

#[allow(unused_variables)]
fn mutating_immutable_variable(){
  // immutable in rust doesn't mean it can't be changed, it is because of exterior mutability

  use std::sync::Arc;
  let x = Arc::new(5);
  let y = x.clone();

  println!("{}", y);
  println!("{}", x);
  //'Arc' stands for 'Atomically Reference Counted'.
  // Arc<T> provides shared ownership of a value of type T
  // invoking clone will create new pointer to same value in heap
}

fn interior_mutability() {
  use std::cell::RefCell;
  let x = RefCell::new(42);
  let y = x.borrow_mut();
  println!("{:?}", x); // it will say that value is borrowed by this rust will prevent violating of its safety polocy
    println!("{:?}", y);
}

#[allow(dead_code)]
fn mutating_struct() {
  use std::cell::Cell;
  // in structs you can't put some variables as mutable and some are not
  struct Point {
    x: i32,
    y: Cell<i32>,
  }
  let point = Point { x: 5, y: Cell::new(6) };
  point.y.set(7);
  println!("y: {:?}", point.y);
}
