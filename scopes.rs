#![allow(unused_variables)]
#![allow(dead_code)]
fn foo() {
  let x = 5; // x scope starts here
  let y = 6; // y scope starts here
} // x and y scopes ends here

fn bar() {
  let foo = 3; //foo scope starts here
  {
    let bar = 5; //bar scope starts here
  } // bar scope ends here
} // foo scope ends here

// fn err_in_scope() {
//   let x; // x scope starts here
//   {
//     let y = 3; // y scope starts here
//     x = &y;
//   } // y scope ends here
//   println!("{:?}",x);
// } // x scope ends here

fn main() {
  // err_in_scope(); //gives an error of "`y` does not live long enough"
}
