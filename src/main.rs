use rand::Rng;
use std::io::Cursor;
use byteorder::{LittleEndian, BigEndian,ReadBytesExt, WriteBytesExt};
use std::{str};
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

  /*
   *
   * Debug	? formatting.
   *
   * let v = vec![1, 2, 3];
   * println(""{:?}", v);
    
   * as of Rust 1.58 you can Debug-format a variable by putting it right after the opening curly bracket, like this:
   * println!("{v:?}");
   * 
   * more info
   * https://doc.rust-lang.org/std/fmt/index.html
   * 
   */
  println!("{:x?}",arr1);
  println!("{arr1:x?}");

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

  // clone data
  let mut arr3 = [0; 512];
  arr3[0..arr2.len()].clone_from_slice(&arr2[0..arr2.len()]);
  println!("{:x?}",arr3);

  // string
  let str1 = "hello"; // &str
  let str2 = str1.to_string(); // String
  let str3 = String::from("hello"); // String
  println!("{},{},{}",str1,str2,str3);

    // for each string
  for c in str1.bytes() {
    println!("{} ",c);
  }

  let x: Vec<u32> = vec![1, 2, 3];
  println!("{x:?}");

  // Vec<u8> to [u8]
  // This is because Vec<T> implements AsRef<[T]>, so &Vec<T> can be coerced into &[T]
  let a = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let b = &a; // b: &Vec<u8>
  let c: &[u8] = &a; // c: &[u8]
  // let c = &a[..]; // c: &[u8]
  ///
  /// let a = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  /// let c: &[u8] = &a; // works because impl Deref for Vec<T> with Target = [T]
  /// let d: &[u8] = a.as_ref(); // works because impl<T> AsRef<[T]> for Vec<T>
  /// 
  
  // &[u8] to str (string slice)
  let buf = &[0x41u8, 0x41u8, 0x42u8];

  let s = match str::from_utf8(buf) {
      Ok(v) => v,
      Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
  };
  println!("{:?}",s);
  
}

/*
 *
 * let x: Vec<u32> = {
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
  };
 * 
 * Matching
 * ( $( $x:expr ),* ) => { ... };
 * 
 * The matcher $x:expr will match any Rust expression, binding that syntax tree to the ???metavariable??? $x. 
 * The identifier expr is a ???fragment specifier???; the full possibilities are enumerated later in this chapter. 
 * Surrounding the matcher with $(...),* will match zero or more expressions, separated by commas.
 * 
 * 
 * $(
    temp_vec.push($x);
   )*

 * That???s most of the matcher syntax. These examples use $(...)*, which is a "zero or more" match. 
 * Alternatively you can write $(...)+ for a "one or more" match. Both forms optionally include a separator, which can be any token except + or *.
 * 
 * 
 * macro_rules! o_O {
    (
        $(
            $x:expr; [ $( $y:expr ),* ]
        );*
    ) => {
        &[ $($( $x + $y ),*),* ]
    }
}

fn main() {
    let a: &[i32]
        = o_O!(10; [1, 2, 3];
               20; [4, 5, 6]);

    assert_eq!(a, [11, 12, 13, 24, 25, 26]);
}

 */
macro_rules! vec {

  ( $( $x:expr ),* ) => {
      {
          let mut temp_vec = Vec::new();
          $(
              temp_vec.push($x);
          )*
          temp_vec
      }
  };
}
