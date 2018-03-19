#![allow(unused_variables)]
//'a is a lifetime parameter which will be explained in lifetime.rs
struct Programmer<'a> {
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
  let age = &19;
  let raghu = Programmer {
    name:"Raghu",
    age : age,
    fav_lang : "Rust, C, Assembly and etc",
    sleeping_hrs: 8
  };

  println!("I am {} , {} years old , As a Programmer I like {} languages.Everyday I sleep for {} hrs.",raghu.name(), raghu.age(),raghu.fav_lang,raghu.sleeping_hrs);
}
