fn print(array: &[i32]) {
  for variable in array {
    println!("{:?}",variable);
  }
}

fn main() {
  // loop {
  //   println!("infinite loop which will stop only at breaking loop by a condition");
  //   println!("similar to while of true but more efficient than it and can take care by compiler");
  // }

  // while expression {
    // println!("same as while in all languages");
  // }
  println!("printing an array using for:");
  let array = [0,1,2,3,4];
  print(&array);

  //prints from 0 to 4 (takes 0 as lower limit and 5 as upper limit but won't print 5)
  println!("printing values using iterating through numbers:");
  for int in 0..5 {
    println!("{:?}",int);
  }

  // enumerate is used to get value and index
  println!("enumerating a for loop:");
  for_enumerate();
  println!("iterating a string for, for loop:");
  for_iterator();
  println!("labeling a for loop:");
  for_labels();
}

fn for_enumerate() {
  for (index,value) in (0..5).enumerate() {
    println!("value at index {} is {}",index, value);
  }
  // can not enumerate an array
  // for (index,value) in array.enumerate() {
  //   println!("value at index {} is {}",index, value);
  // }
}

fn for_iterator() {
  let lines = "hello\nworld".lines();
  for (linenumber, line) in lines.enumerate() {
      println!("line at line number {} is {}", linenumber, line);
  }
}

fn for_labels(){
  'outer: for x in 0..5 {
    'inner: for y in 0..5 {
        if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
        if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
        println!("x: {}, y: {}", x, y);
    }
  }
}
