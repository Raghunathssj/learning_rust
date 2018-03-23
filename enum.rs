#![allow(unused_variables)]
#![allow(dead_code)]
fn description() {
  // enum is a type that represents data with several possible variants
  // it can resembles any kind of variants which has, which din't have data
  // and which has named and which has unnamed data
  // if you asking for is a value present in it, if it present it will check for every  variant in it
  // that's why it is called "sum type"
  // each variant scope is only on its enum
}

fn main() {
  description();
  enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
  }
  let point = Message::Move{x:1,y:2};
  //enum also contains information about its variant and data associated with it
  // it is referred as "tagged union"
  // compiler uses this data for safety of data in enum

  // fn process_color_change(msg: Message) {
  //   let Message::ChangeColor(r, g, b) = msg; // This causes a compile-time error.
  // }
  let message = Message::Write("hello".to_string());
  // the above line can also be written as
  fn foo(x: String) -> Message {
    Message::Write(x)
  }

  let x = foo("Hello, world".to_string());
  // the above kind of declarations can be useful on writing closures

  // by using iterators we can convert vector of strings to vector of message Writes
  let v = vec!["Hello".to_string(), "World".to_string()];
  let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
}
