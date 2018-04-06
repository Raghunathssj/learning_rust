#![allow(dead_code)]

fn description() {
  // Traits are a way to tell compiler about a functionality that a type must implement
  // It only has type signature of the method that needs to implement
  // It won't contain any defination in it
  // The types that use this trait needs to define these methods
  // self may use in trait to refer the instance of a type that is implementing
  // self,&self and &mut self may used depending on level of ownership needed
}

fn main() {
  description();
  example();
  multiple_signatures_in_trait();
  // trait_bounds_on_generic_functions();
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
  unimplemented!()
}
