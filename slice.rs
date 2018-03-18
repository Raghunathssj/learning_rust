fn main() {
  let array = [1,2,3,4,5,6];
  let complete_slice = &array[..];
  let middle_slice = &array[1..4];
  println!("original array");
  for oa in &array {
    println!("{:?}",oa);
  }
  println!("completely sliced array");
  for c in complete_slice {
    println!("{:?}",c);
  }
  println!("part sliced array");
  for p in middle_slice {
    println!("{:?}",p);
  }
  let vec = vec![1, 2, 3];
  let int_slice = &vec[1..2];
  println!("original vector");
  for ov in &vec {
    println!("{:?}",ov);
  }
  println!("sliced vector");
  for v in int_slice {
    println!("{:?}",v);
  }
  let str_slice: &[&str] = &["one", "two", "three"];
  println!("coercing an array to a slice");
  for s in str_slice {
    println!("{:?}",s);
  }
}
