use rand::Rng;
use std::io::Cursor;
use byteorder::{LittleEndian, BigEndian,ReadBytesExt, WriteBytesExt};
use std::{fmt, result, str};
/*
 *
 * By default, arrays are immutable.
 * 
 * let a = [1, 2, 3]; // a: [i32; 3]
 * let mut m = [1, 2, 3]; // mut m: [i32; 3]
 * There's a shorthand for initializing each element of an array to the same value. In this example, each element of a will be initialized to 0:
 *
 * let a = [0; 20]; // a: [i32; 20]
 *
 * You can get the number of elements in an array a with a.len(), and use a.iter() to iterate over them with a for loop. This code will print each number in order:
 *
 * let a = [1, 2, 3];
 *
 * println!("a has {} elements", a.len());
 * for e in a.iter() {
 *   println!("{}", e);
 * }
 * 
 * Subscripts start at zero, like in most programming languages, so the first a is a[0]
 * 
 * 
 * 
 * 
 * A vector is a dynamic or "growable" array.
 * Vectors always allocate their data on the heap. Vectors are to slices what String is to &str.
 * 
 * You can get the length of, iterate over, and subscript vectors just like arrays. In addition, (mutable) vectors can grow automatically:

 * let mut nums = vec![1, 2, 3]; // mut nums: Vec<i32>
 * nums.push(4);
 * println!("The length of nums is now {}", nums.len()); // Prints 4
 * 
 * 
 * A slice is a reference to (or "view" into) an array.
 * let a = [0, 1, 2, 3, 4];
 * let middle = &a[1..4]; // A slice of a: just the elements 1, 2, and 3
 * 
 * for e in middle.iter() {
 *  println!("{}", e); // Prints 1, 2, 3
 * }
 * 
 * You can also take a slice of a vector, String, or &str, because they are backed by arrays.
 * 
 */

fn read_string(bytes: &[u8], start: usize) -> (String, usize) {
  let mut result_bytes = Vec::new();
  let mut counter = start;
  
  while bytes[counter] != 0 {
      result_bytes.push(bytes[counter]);

      counter += 1;
      if counter >= bytes.len() {          
          return ("".to_string(),0);
      }
  }
  counter += 1;

  let result_str = str::from_utf8(result_bytes.as_slice()).unwrap().to_string();
  (result_str, counter)
}

fn main() {

/*  
  let mut rng = rand::thread_rng();

  let n1: u8 = rng.gen();
  println!("Random u8: {}", n1);
  println!("Random u32: {}", rng.gen::<u32>());
  
  //print with range[1 to 4]
 	let n2: u8 = rand::thread_rng().gen_range(1..5);
*/
  let mut rng = rand::thread_rng();
  let mut arr1 = [0u8; 128];
  //0 ~ 127
  for i in 0..128 {
    arr1[i] = rng.gen();    
  }

  println!("{:x?}",arr1);

  // merge u8 -> u16
  let mut m16 = Cursor::new(vec![arr1[0], arr1[1]]);
  let u16 = m16.read_u16::<BigEndian>().unwrap();
  println!("{:x?}",u16);

  // merge u8 -> u32
  let mut m32 = Cursor::new(vec![arr1[0], arr1[1], arr1[2], arr1[3]]);
  let u32 = m32.read_u32::<LittleEndian>().unwrap();
  println!("{:x?}",u32);
  

  // u16 -> u8
  let mut wtr = vec![];
  wtr.write_u16::<BigEndian>(u16).unwrap();

  println!("{:x},{:x}",wtr[0], wtr[1]);

  let arr2 = [0,1,0x68,0x65,0x6C,0x6C,0x6F,0x20,0x77,0x6F,0x72,0x6C,0x64,0,1,2,3,4,5];
  let (str, end_pos) = read_string(&arr2, 2);
  println!("{}, end_pos:{}",str, end_pos);

}