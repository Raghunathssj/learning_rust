#![allow(unused_variables)]

fn describe(){
  //Structs are a way to create more compllex datea types
  //Those are immutable by default
  //'a is a lifetime parameter which is be explained in lifetime.rs
}
struct Programmer<'a> {
  //mut name: &'a str, // you can't do like this because of immutability of structs
  name: &'a str,
  age: &'a u8,
  fav_lang: &'a str,
  sleeping_hrs: u8
}

//implementing a method on struct

impl<'a> Programmer<'a> {
    fn name(&self) -> &'a str { self.name }
}

impl<'a> Programmer<'a> {
    fn age(&self) -> &'a u8 { self.age }
}

fn main() {
  describe();
  let age = &19;
  let raghu = Programmer {
    name:"Raghu",
    age : age,
    fav_lang : "Rust, C, Assembly and etc",
    sleeping_hrs: 8
  };
  println!("I am {} , {} years old , As a Programmer I like {} languages.Everyday I sleep for {} hrs.",raghu.name(), raghu.age(),raghu.fav_lang,raghu.sleeping_hrs);
  mutate_struct();
  reference_mutate_struct();
  update_syntax_struct();
  tuple_struct();
  unit_like_struct();
}

fn mutate_struct(){
  // even though structs are immutable but you can do like this
  let name = "Ram";
  let age = &19;
  let fav_lang = "anything";
  // if your field names and variable names are same than you can do like below
  let mut ram = Programmer {name,age,fav_lang,sleeping_hrs:10};
  // now you can do like this
  ram.sleeping_hrs = 11;
  println!("I am {} , {} years old , As a Programmer I like {} languages.Everyday I sleep for {} hrs.",ram.name(), ram.age(),ram.fav_lang,ram.sleeping_hrs);
}

struct Point {
  x: i32,
  y: i32,
}

struct PointRef<'a> {
  x: &'a mut i32,
  y: &'a mut i32,
}

fn reference_mutate_struct() {
  // But struct still contain refernece of mut
  let mut point = Point { x: 0, y: 0 };
  println!("x is {} and y is {}", point.x,point.y);
    {
      let r = PointRef { x: &mut point.x, y: &mut point.y };

      *r.x = 5;
      *r.y = 6;
    }
  println!("x is {} and y is {}", point.x,point.y);
}

fn update_syntax_struct() {
  struct Point3d {
    x: i32,
    y: i32,
    z: i32,
  }
  let mut point = Point3d { x: 0, y: 0, z: 0 };
  println!("{} {} {}",point.x,point.y,point.z);
  // .. is used to copy some fields from another struct like the below
  point = Point3d { y: 1, .. point };
  println!("{} {} {}",point.x,point.y,point.z);
  let origin = Point3d { x: 0, y: 0, z: 0 };
  let another_point = Point3d{ z: 5, x: 4, ..origin };
  println!("{} {} {}",another_point.x,another_point.y,another_point.z);
}

fn tuple_struct() {
  struct Color (i32,i32,i32);
  struct Point (i32,i32,i32);
  let red = Color(99,0,0);
  let origin = Point(0,0,0);
  //Just like tuple you can access its values using dot
  let r = red.0;
  // you can assign values like this also
  let Point(_, origin_y, origin_z) = origin;
  // this kind of structure will be discussed in match

  // If a tuple struct only has one element it is called 'newtype' patern
  struct Variable (i32);
  let var = Variable(3);
  let Variable(value) = var;
  // the above line can also be written as "let value = var.0" both are same
  println!("{}", value);
  // instead on using "tuple struct" using "struct" is better
  // because it has values with keys, we can access them using their keys instead of positions
}

fn unit_like_struct() {
  struct Empty {};
  struct null;
  // you can create structs like this also, it is called "unit like" or "unit"
  // it is same as implementing empty tuple
  // it may only useful to itself but with other feature it is more useful
}
