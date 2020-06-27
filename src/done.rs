//! this module contains codes already studied
//! # Examples
//! ```
//! pub fn scope() {
//!   let a = 1;
//!   println!("a = {}", a);
//!   {
//!   let a = 10;
//!     println!("a = {}", a);
//!   }
//! }
//! ```

use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;

/// this func is numbers obviously
pub fn numbers() {
  let a: u8 = 123;
  println!("a = {}", a);

  // a = 456;

  let mut b: i8 = 100;
  println!("b = {}", b);
  b = 10;
  println!("b = {}", b);

  let mut c = 123456789;
  println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
  c = -1;

  let d = c;
  println!("c = {}, d = {}", c, d);

  let d: char = 'x';
  println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

  let mut e: u8 = 2 + 3 * 4;
  println!("e = {}", e);
  e += 1;
  println!("e = {}", e);

  let f = 2.5;
  let cubed = f64::powi(f, 4);
  println!("cubed = {}", cubed);
}

pub fn scope() {
  let a = 1;
  println!("a = {}", a);
  {
    let a = 10;
    println!("a = {}", a);
  }
}

struct Point {
  x: f64,
  y: f64,
}

pub fn match_and_memory() {
  let p1 = Point { x: 1.1, y: 2.2 };
  println!("x: {}, y: {}", p1.x, p1.y);

  let pointer = Box::new(Point { x: 0.1, y: 0.2 }); // HEAP
  println!("{}, {}", (*pointer).x, (*pointer).y);

  let country_code = 1000;
  let country = match country_code {
    44 => "UK",
    46 => "Sweden",
    1..=40 => "unknown",
    _ => "invalid",
  };

  println!("country_code: {}, country: {}", country_code, country);
}

pub fn string_loop() {
  let mut str = String::new();
  loop {
    str.push_str("a");
    println!("str: {}", str);

    if str.len() > 10 {
      break;
    };
  }
}

enum Color {
  Red,
  Green,
  Blue,
  RgbColor { r: u8, g: u8, b: u8 },
}

pub fn enums() {
  let c: Color = Color::RgbColor { r: 0, g: 0, b: 0 };

  match c {
    Color::Red => println!("r"),
    Color::Green => println!("g"),
    Color::Blue => println!("b"),
    Color::RgbColor { r: 0, g: 0, b: 0 } => println!("black"),
    Color::RgbColor { r, g, b } => println!("{}, {}, {}", r, g, b),
  }
}

union IntOrFloat {
  i: i32,
  f: f32,
}

pub fn union() {
  let iof = IntOrFloat { i: 42 };
  // iof.i = 234;
  let value = unsafe { iof.i };
  println!("iof.i = {}", value);

  unsafe {
    match iof {
      IntOrFloat { i: 42 } => println!("42!!!!"),
      IntOrFloat { f } => println!("float: {}", f),
    }
  }
}

pub fn handle_none() {
  let x = 10.0;
  let y = 2.5;
  let result = if y != 0.0 { Some(x / y) } else { None };

  match result {
    Some(z) => println!("{}/{} = {}", x, y, z),
    None => println!("cannot devide by zero"),
  }

  if let Some(z) = result {
    println!("result = {}", z);
  }
}

pub fn array() {
  let mut a: [i32; 5] = [1, 2, 3, 4, 5];
  println!("a has {} elements, first is {}", a.len(), a[0]);
  a[0] = 100;
  println!("a has {} elements, first is {}", a.len(), a[0]);
  println!("{:?}", a);

  if a != [1, 2, 3, 4, 5] {
    println!("modified");
  }

  let b = [a; 10];

  for i in 0..b.len() {
    println!("i: {:?}", b[i]);
  }
}

fn use_slice(slice: &mut [i32]) {
  println!("first elem = {}, len = {}", slice[0], slice.len());
  slice[0] = 0;
}

pub fn slice() {
  let mut data = [1, 2, 3, 4, 5];
  use_slice(&mut data);
  println!("{:?}", data);
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
  (x + y, x * y)
}

pub fn tupple() {
  let x = 3;
  let y = 10;
  let sp = sum_and_product(x, y);
  println!("sp = {:?}, 0 = {}, 1 = {}", sp, sp.0, sp.1);
  let sp2 = (sp, sp);
  println!("{:?}", sp2);

  let (a, b) = sp2;
  println!("{:?}, {:?}", a, b);

  let single_tupple = (42,);
  println!("{:?}", single_tupple);
}

fn how_many(x: i8) -> &'static str {
  match x {
    0 => "no",
    1 | 2 => "one or two",
    12 => "dozen",
    9..=11 => "lots of ",
    _ if (x % 2 == 0) => "some",
    _ => "a few",
  }
}

pub fn pattern_matching() {
  for x in 0..13 {
    println!("{}: I have {} oranges!", x, how_many(x));
  }

  let point = (3, 0);

  match point {
    (0, 0) => println!("origin!"),
    (0, y) => println!("x axis, y {}", y),
    (x, 0) => println!("x {}, y axis", x),
    (x, y) => println!("{}, {}", x, y),
  };
}

struct PointWith<T> {
  x: T,
  y: T,
}

pub fn generics() {
  let a: PointWith<u16> = PointWith { x: 1, y: 1 };
  let b: PointWith<f32> = PointWith { x: 1.0, y: 1.0 };
}

pub fn vectors() {
  let mut a = Vec::new();
  a.push(1);
  a.push(2);
  a.push(3);
  println!("{:?}", a);

  a.push(44);
  println!("{:?}", a);

  let index: usize = 0;
  println!("a[0] = {}", a[index]);

  match a.get(1) {
    Some(z) => println!("{}", z),
    None => println!("not exist"),
  }

  println!("{:?}", a);

  let last_elm = a.pop();
  println!("last element is {:?}, {:?}", last_elm, a);

  while let Some(x) = a.pop() {
    println!("{}", x);
  }
}

pub fn hash_map() {
  let mut shapes = HashMap::new();
  shapes.insert(String::from("triangle"), 3);
  shapes.insert("square".into(), 4);

  for (key, value) in &shapes {
    println!("{} {}", key, value);
  }

  shapes.entry("circle".into()).or_insert(0);
  shapes.entry("circle".into()).or_insert(1);

  println!("{:?}", shapes);
}

pub fn hash_set() {
  let mut greeks = HashSet::new();
  greeks.insert("gamma");
  greeks.insert("delta");
  println!("{:?}", greeks);

  greeks.insert("delta");
  println!("{:?}", greeks);

  let added_vega = greeks.insert("vega");
  if added_vega {
    println!("added!");
  }

  if !greeks.contains("kappa") {
    println!("kappa not included");
  }

  let _1_5: HashSet<_> = (1..=5).collect();
  let _6_10: HashSet<_> = (6..=10).collect();
  println!("{:?}", _1_5);
  println!("{:?}", _6_10);

  println!("{}", _1_5.is_subset(&_6_10));
}

pub fn strings() {
  let s: &'static str = "Hello World!";
  println!("{}", s);

  let mut letters = String::new();
  let mut a = 'a' as u8;
  while a <= ('z' as u8) {
    letters.push(a as char);
    letters.push_str(",");
    a += 1;
  }

  println!("{}", letters)
}
