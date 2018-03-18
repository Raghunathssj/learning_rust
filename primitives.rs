fn main() {
  let y: bool = false;
  let character: char = 'x';
  let symbol: char = '😎';
  let unsigned_int8: i8 = 127;
  let unsigned_int16: i16 = 32767;
  let unsigned_int32: i32 = 2147483647;
  let unsigned_int64: i64 = 9223372036854775807;
  let signed_int8: u8 = 255;
  let signed_int16: u16 = 65535;
  let signed_int32: u32 = 4294967295;
  let signed_int64: u64 = 18446744073709551615;
  let float32: f32 = 1234.5678;
  let float64: f64 = 12345678.91234567;
  // let array: [i32;3] = [1,2,3]
  /// usize and isize are varies in sizes depends in machine
  println!("boolean: {:?}",y);
  println!("character: {:?}",character);
  println!("symbols can also be on character {:?}",symbol);
  println!("unsigned int of 8 bits {:?}",unsigned_int8);
  println!("unsigned int of 16 bits {:?}",unsigned_int16);
  println!("unsigned int of 32 bits {:?}",unsigned_int32);
  println!("unsigned int of 64 bits {:?}",unsigned_int64);
  println!("signed int of 8 bits {:?}",signed_int8);
  println!("signed int of 16 bits {:?}",signed_int16);
  println!("signed int of 32 bits {:?}",signed_int32);
  println!("signed int of 64 bits {:?}",signed_int64);
  println!("float of 32 bits which will hold signed or unsigned 8 digits at total with out '.' {:?}",float32);
  println!("float of 64 bits which will hold signed or unsigned 16 digits at total with out '.' {:?}",float64);
}
