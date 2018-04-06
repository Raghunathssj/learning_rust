#![allow(unused_variables)]
#![allow(dead_code)]

fn description() {
  // Generics are called "Parametric Polymorphism" in type theory
  // Which means that they are types or functions of multiple forms over a given parameter ("Parametric" - "parameter","Poly" - "Multiple" ,"Morph" - "Forms")
  // You can use any capital letter(Convention) to specify while declaring any generic
}

fn main() {
  description();
  standard_generic_option();
  standard_multiple_generics_option();
  generic_functions();
  generic_structs();
  generic_vectors();
}

// #########################################
fn standard_generic_option() {
  enum Option<T> {
      Some(T),
      None,
  }
  let x: Option<i32> = Option::Some(5); // Here type of x is i32
  // let y: Option<i32> = Option::Some(5.0f64); // Here we initialized Option as i32 but we are giving float value this gives error
}

// ########################################
fn standard_multiple_generics_option() {
  enum Result<T, E> {
    Ok(T),
    Err(E),
  }
  // Here it is a generic of two types
  // This is used return the result of computation, and it can return error if it don't work
}

// #######################################
fn generic_functions() {
  single_argument_generic_function(3); // Here we are making type as i32 (default)
  single_argument_generic_function(3.044f64); // Here we are making type as f64
  two_arguments_of_same_type_generic_function(3,5);
  // two_arguments_of_same_type_generic_function(3,5.034f64); // If you try to pass deferent types to this function it will throw error
  two_arguments_of_different_type_generic_function(3,5.330f64);
}
use std::fmt::Debug;
fn single_argument_generic_function<T: Debug>(x:T) {
  // Here we need to use "Debug" to print the value otherwise it will throw error
  println!("{:?}", x);
}

fn two_arguments_of_same_type_generic_function<T: Debug>(x:T, y:T) {
  println!("{:?} and {:?}", x,y);
}

fn two_arguments_of_different_type_generic_function<X: Debug,Y:Debug>(x:X, y:Y) {
  // If you are passing two different types, you should mention "Debug" in both places otherwise it will throw error
  println!("{:?} and {:?}", x,y);
}

// #######################################

fn generic_structs() {
  // Similar to functions we will declare generic parameters using <T>
  struct Point<T> {
    x: T,
    y: T,
  }
  let int_origin = Point { x: 0, y: 0 };
  let float_origin = Point { x: 0.0, y: 0.0 };
  // You can also create a generic impl by specificing type parameter after impl
  impl<T> Point<T> {
    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
  }
}

fn generic_vectors() {
  let mut v = Vec::new();
  // println!("{:?}", v); // This will give you an error because :? cannot resolve type of v
  // If you use like this it will work
  v.push(true); // This will make the type v as bool thats why the next line won't give error
  println!("{:?}", v);
}
