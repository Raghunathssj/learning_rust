#![allow(unused_variables)]
#![allow(dead_code)]
fn description() {
  //Match is similar to groups of if else cases
  //using this you can remove all if elses and use a simple syntax for match
  //Bassically match is a group of arms which is same as branches in if/else
  // it matches the given parameter accourding to its pattern
  // that's why it is called match because of its pattern matching
}

fn main() {
  description();
  // every arm of match is seperated with ","
  // while you are using match at last you should use "_" other wise it will give error
  match "hello" {
    "hai" => println!("hai"),
    "hello" => println!("hello"),
    "good" => println!("good"),
    _ => println!("nope it is not hello")
  }
  // you can also use variable in match
  let x = 0;
  match x {
    1 => println!("Its one"),
    2 => println!("Its two"),
    3 => println!("Its three"),
    _ => println!("Its something")
  }
  // If no pattern matching is present the "_" will be executed
  matching_enums();
  //different kind of patterns that can be used in match will be coverd in pattern.rs
}


fn matching_enums() {
  let quit = Message::Quit;
  process_message(quit);
  let write = Message::Write("Hello".to_string());
  process_message(write);
}

enum Message {
  Quit,
  ChangeColor(i32, i32, i32),
  Move { x: i32, y: i32 },
  Write(String),
}

fn quit() {  }

fn change_color(r: i32, g: i32, b: i32) {  }

fn move_cursor(x: i32, y: i32) {  }

fn process_message(msg: Message) {
  // for matching enums you should mention all variants of the enum
  // or you can use _ for some variants
  // if you are mentioning all variants you don't need to mention "_"
  match msg {
    Message::Quit => quit(),
    Message::ChangeColor(r, g, b) => change_color(r, g, b),
    Message::Move { x, y: new_name_for_y } => move_cursor(x, new_name_for_y),
    Message::Write(s) => println!("{}", s),
  };
}
