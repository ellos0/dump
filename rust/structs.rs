#![allow(dead_code)]

#[derive(Debug)]

struct Guy {
  name: String,
  age: u8,
}

struct Unit;

struct Point {
   x: i32,
   y: i32,
}

struct Rectangle {
   top_left: Point,
   bottom_right: Point,
}

fn main() {
   println!("yay");
}