#![allow(unused_variables)]
struct Programmer<'a> {
  name: &'a str,
  age: u8,
  fav_lang: &'a str,
  sleeping_hrs: u8
}

fn main() {
  let raghu = Programmer {
    name:"Raghu",
    age : 19,
    fav_lang : "Rust, C, Assembly and etc",
    sleeping_hrs: 8
  };

  println!("I am {} , {} years old , As a Programmer I like {} languages.Everyday I sleep for {} hrs.",raghu.name, raghu.age,raghu.fav_lang,raghu.sleeping_hrs);
}
