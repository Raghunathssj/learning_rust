#![allow(dead_code)]
#![allow(unused_variables)]

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
  traits_for_int();
  multiple_trait_bounds();
  where_clause_in_traits();
  default_methods_in_traits();
  overriding_default_methods_in_traits();
  inheritance();
  deriving_traits();
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
  // If you use a generic function it may complains because it cannot find scope of the methods that are using in the generic function like area() in below example
  /*
  fn print_area<T>(shape: T) {
    println!("This shape has an area of {}", shape.area());
  }
  */
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

// #######################################
fn traits_for_int() {
  trait IsSame {
    fn is_same(&self, other: &Self) -> bool;
  }
  impl IsSame for i32 {
    fn is_same(&self, other: &Self) -> bool {
      self == other
    }
  }
  println!("{}",3.is_same(&3));
  println!("{}",3.is_same(&5));
  // Like this you can make your own traits for any type.
}

// #######################################
fn multiple_trait_bounds() {
  //We are using single traits till now, what if we want to use more than one trait
  //Then you can use "+" for that
  use std::fmt::Debug;
  // Clone is also standard trait
  fn foo<T: Clone + Debug>(x: T) {
    x.clone();
    println!("{:?}", x);
  }
  foo(3);
}

// #######################################
fn where_clause_in_traits() {
  // If you want to use different traits for different types or same type of data that is taking it is possible and it seems like below
  use std::fmt::Debug;
  fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
  }
  // Here the type T using only Clone trait and type K is using Clone and Debug traits
  // It seems that the function name and the parameters are at far distance and its is hard to see them to solve this Rust has "where clause"
  fn bar<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
    // While using "where" we use the function syntax and use where after parameters
    x.clone();
    y.clone();
    println!("{:?}", y);
  }
  // You can also break the line if the parameters or traits are more to increase readability
  fn foo_bar<T,K>(x: T,y: K)
    where T:Clone,
          K: Clone + Debug {
            // some code....
  }


  // Here is the another feature of "where" is, it allow bound to left side of it not only of type parameters
  // Meaning, "where" bounds according to the position it not only bounds to the parameter type (above example)
  trait ConvertTo<Output> {
    fn convert(&self) -> Output;
  }
  impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 { *self as i64 }
  }
  // Can be called with T == i32.
  fn convert_t_to_i64<T: ConvertTo<i64>>(x: T) -> i64 {
    x.convert()
  }
  // Can be called with T == i64.
  fn convert_i32_to_t<T>(x: i32) -> T
        // This is using ConvertTo as if it were "ConvertTo<i64>".
        where i32: ConvertTo<T> {
    x.convert()
  }
}

// #######################################
fn default_methods_in_traits() {
  // In traits we can add define default method
  trait EaqulExample {
    fn are_equal(&self,other: &Self) -> bool;
    fn are_not_equal(&self,other: &Self) -> bool {!self.are_equal(other)}
  }
  // In the above trait we have declared an default method, it won't force to create are_not_equal method while implementing this trait
  impl EaqulExample for i32 {
    fn are_equal(&self, other: &Self) -> bool {
      self == other
    }
  }
  // The above implementation won't give any compilation error
}

// #######################################
fn overriding_default_methods_in_traits() {
  trait EaqulExample {
    fn are_equal(&self,other: &Self) -> bool;
    fn are_not_equal(&self,other: &Self) -> bool {!self.are_equal(other)}
  }
  impl EaqulExample for i32 {
    fn are_equal(&self, other: &Self) -> bool {
      self == other
    }
    fn are_not_equal(&self,other: &Self) -> bool {
      self != other
    }
    // Here we are overriding the default method
  }
}

// #######################################
fn inheritance() {
  trait Ex {
    fn something();
  }
  trait Example : Ex {
    fn some();
  }
  // Here we are inheriting Ex to Example
  // That means who ever implements Example should implement Ex also
  struct Foo;
  impl Ex for Foo {
    fn something() {println!("something");}
  }
  impl Example for Foo {
    fn some() {println!("some",);}
  }
  // You can't implement two traits at a time
  // Even though the traits are inherited you need to implement them seperately
  /*
  impl Example for Foo{
    fn something() {println!("something");}
    fn some() {println!("some",);}
  }
  */
  // If you try to implement like the above code compiler will throw error
  // It will say something method is not belongs to this trait
}

// #######################################
fn deriving_traits() {
  // implementing standard traits repeatedly makes you frustated.
  // That's why Rust introduced #[derive(Trait)] to use below traits to include it automatically
  /*
  Clone
  Copy
  Debug
  Default
  Eq
  Hash
  Ord
  PartialEq
  PartialOrd
  */
  #[derive(Debug)]
  // Here we are using derive in Foo for Debug because a struct can't be debugged directly using :?
  struct Foo;
  fn main() {
    println!("{:?}", Foo);
  }
}
