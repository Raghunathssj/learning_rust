#![allow(unused_variables)]
fn description() {
  // there are 2 types of strings in rust String and &str which is called string slices
  // A ‘string’ is a sequence of Unicode scalar values encoded as a stream of UTF-8 bytes.
  // A string slices are fixed size and immutable but it is unsized variable
  // in rust strings are not null terminated and can use null characters in strings
}

fn main() {
  description();
  string_literals();
  strings();
}

fn string_literals() {
  let string_slice = "string slice"; // it is same as declaring string_slice: &'static str
  // here "string slice" is string literal, which is a string slice that is statically allocated
  // any function that accepts string slices will also accepts string literals
  //string literals can also span multiple lines
  // there are 2 types
  // first is with that have new line and with spaces
  let multiple_with_spaces = "multiple lines
        spaces";
  println!("{}",multiple_with_spaces);
  // second with "\" which trims new line and spaces
  let multiple_with_backslash = "multiple lines \
        with back slash";
  println!("{}",multiple_with_backslash);
  // you can't access a str directly, you need to use &str because it is unsized variable
}

fn strings() {
  //String can be commonly created using string slice with to_string function
  // Strings can join with str
  let mut s = "join".to_string();
  println!("{}", s);
  s.push_str(" it");
  println!("{}", s);
  string_coercing();
  string_indexing();
  string_slicing();
  string_concatenation();
}

fn string_coercing() {
  //you can also pass String to a function as &str by coercing(forcing) it
  let s = "string".to_string();
  // example(&s);
  // but for some function that accepts &str trait instead of str coercing won't happen
  // you can pass using &* to that function
  // use std::net::TcpStream;
  // TcpStream::connect("192.168.0.1:3000"); // Parameter is of type &str.
  // let addr_string = "192.168.0.1:3000".to_string();
  // TcpStream::connect(&*addr_string); // Convert `addr_string` to &str.
  // the above function will take & str but you need to cast String using &*
}

fn string_indexing() {
  // Like in other languages you can't access string letters using [] or it can't be indexed using integer
  // println!("The first letter of s is {}", "hello"[0]); // It will give error
  // But you can access letters in string like this

  let dog = "hello".chars().nth(0); // Its like "hello"[0]
}

fn string_slicing() {
  let hello = "hello";
  let string_slice = &hello[0..3];
  println!("{}", string_slice);
  // But it will fail in run time because strings are not character offcets those are byte offcets
}

fn string_concatenation() {
  // you can concat string and literal string easily
  let string = "string".to_string();
  let concat = "concat";
  let string_concat = string + concat;
  println!("{}",string_concat);
  // If you try to add two strings it will give error
  // You can add two strings using & symbol like below
  let string = "string".to_string();
  let concat = "concat".to_string();
  let string_concat = string + &concat;
  println!("{}",string_concat);
}
