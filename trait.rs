#![allow(dead_code)]

fn description() {
  // Traits are a way to tell compiler about a functionality that a type must implement
  // It only has type signature of the method that needs to implement
  // It won't contain any defination in it
  // The types that use this trait needs to define these methods
  // self may use in trait to refer the instance of a type that is implementing
  // self,&self and &mut self may used depending on level of ownership needed
  // You can implement trait for every method
}

fn main() {
  description();
  example();
  multiple_signatures_in_trait();
  trait_bounds_on_generic_functions();
  trait_bounds_on_generic_structs();
}

// #######################################
fn example() {
  // This is a circle struct which we are going to implement a trait
  struct Circle {
    radius: f64,
  }
  // This is the syntax of the trait
  trait HasArea {
    // Here we are created a signature of area function that circle needs to implement if it uses this trait
    fn area(&self) -> f64;
  }
  // while implement a trait the syntax of impl is impl TraitName for ItemName
  // If we won't implement any function that signatured in trait it will throw error
  impl HasArea for Circle {
    // Here we are implement the functions that trait has
    fn area(&self) -> f64 {
      //Here we are using standard float constant PI
        std::f64::consts::PI * (self.radius * self.radius)
    }
  }
}

// #######################################
fn multiple_signatures_in_trait() {
  // It is same as implementing a trait with single signature but has more than one signature
  trait HasArea {
      fn area(&self) -> f64;
      fn is_larger(&self, &Self) -> bool;
      // Here &Self represents the refernence of other instance
      //  for that you need to define like
      // fn is_larger(&self, other: &Self) -> bools
  }
  // while implementing this trait we need to define every method that is signatured in trait
}

// #######################################
fn trait_bounds_on_generic_functions() {
  /* If you use a generic function it may complains because it cannot find scope of the methods that are using in the generic function like area() in below example
  fn print_area<T>(shape: T) {
    println!("This shape has an area of {}", shape.area());
  }*/
  // Here T can be any type and we don't know that, the type may implement area method or not.
  trait HasArea {
    fn area(&self) -> f64;
  }
  // Here we are bounding trait to the generic function thats why it won't give eny error
  fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
  }
  // Here <T: HasArea> means the method that implements HasArea traits only
  struct Circle {
    radius: f64,
  }
  impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
  }
  let c = Circle{
    radius: 1f64
  };
  print_area(c);
  // If you try to give any type other than the type that has area function to generic function(print_area) it will give error
  // print_area(1); // this line will give error that it expects HasArea trait for {integer}
}

// #######################################
fn trait_bounds_on_generic_structs() {
  // You can append trait bound when you declare type parameters
  // Here we have a struct Rectangle which we implementing is_square method in it
  struct Rectangle<T> {
    width: T,
    height: T,
  }
  // Here we are checking the equality between the width and height
  // to check that the types of sides must implements core::cmp::PartialEq trait
  impl<T: PartialEq> Rectangle<T> {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
  }
  // Using <T:PartialEq> Rectangle can be defined any type that can compared equality
  let r = Rectangle {
    width:1,
    height:1
  };
  println!("{:?}", r.is_square());
  // You can also use standard traits instead of HasArea for Circle but it need multiplication
}
