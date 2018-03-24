#![allow(unused_variables)]
#![allow(dead_code)]

fn desctription() {
  //You can create a method using impl
  // Rust will provide "Method Call Syntax" through impl
  // all methods can take first parameter as self,&self and &mut self
}

struct Programmer<'a> {
  name: &'a str,
  age: u8
}

// here we are creating methods in Programmer
// You can create many methods in one impl
impl<'a> Programmer<'a  > {
  fn new(name: &'a str,age:u8) -> Programmer{
     Programmer {name,age}
  }
  fn get_name(&self) -> &'a str {
    self.name
  }
  fn get_age(&self) -> u8{
    self.age
  }
}

// or you can create methods using different impls
impl<'a> Programmer<'a> {
  fn update_name(&mut self,name: &'a str) -> &'a str {
    self.name=name;
    self.name
  }
  fn change(&self,name: &'a str) -> Programmer {
    Programmer {name:name,age:self.age}
  }
}

fn main() {
  desctription();
  method_call();
  chaining_method_calls();
  associated_methods();
  builder_pattern_method();
}

fn method_call() {
  let mut programmer = Programmer{name:"Raghu",age:19};
  println!("name is {}",programmer.get_name());
  println!("age is {:?}",programmer.get_age());
  println!("updated name is {}",programmer.update_name("Raghunath"));
}

// you can chain the function calling like below
fn chaining_method_calls() {
  let programmer = Programmer {name:"Raghu",age:19};
  println!("name is {}",programmer.get_name());
  // here it is working because we are returning a new Programmer in change method
  // and we are calling get name function on that returned Programmer
  let programmer1_name = programmer.change("Ram").get_name();
  println!("changed name is {}",programmer1_name);
}

fn associated_methods() {
  // you can also create a methods which won't take self argument like the new method in above struct
  let programmer = Programmer::new("Raghu",19);
  // here we need to call Programmer method like struct::method() to create a new Programmer because there is no reference to call the method in it
  println!("{} is {} years old",programmer.get_name(),programmer.get_age());
}

fn builder_pattern_method() {
  // here we created a struct with name Circle which has a method area in it
  struct Circle {
    x: f64,
    y: f64,
    radius: f64,
  }
  impl Circle {
    fn area(&self) -> f64 {
      std::f64::consts::PI * (self.radius * self.radius)
    }
  }
  // here we created a struct that builds Circle using the values that given by user which he/she wants to set and remaining are setted as default values
  struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
  }
  impl CircleBuilder {
    // this method creates a new circle builder with default values
    fn new() -> CircleBuilder {
      CircleBuilder { x: 0.0, y: 0.0, radius: 1.0, }
    }
    // by using this method user can give his prefferd values
    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
      self.x = coordinate;
      self
    }
    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
      self.y = coordinate;
      self
    }
    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
      self.radius = radius;
      self
    }
    // this method will create final Circle with the values that are in it and returns it
    fn finalize(&self) -> Circle {
      Circle { x: self.x, y: self.y, radius: self.radius }
    }
  }

  let c = CircleBuilder::new()
                .x(1.0)
                .y(2.0)
                .radius(2.0)
                .finalize();
    println!("area: {}", c.area());
    println!("x: {}", c.x);
    println!("y: {}", c.y);
}
