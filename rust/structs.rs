#![allow(dead_code)]

#[derive(Debug)]

struct Guy {
  name: String,
  age: u8,
}

struct Unit;

struct Point {
   x: f32,
   y: f32,
}

struct Rectangle {
   top_left: Point,
   bottom_right: Point,
}

fn main() {
  let name = String::from("Peter");
  let age = 27;
  let peter = Guy {name,age};
  
  println!("{:?}", peter);
  
  let point: Point = Point {x: 5.2, y: 0.4};
  let another_point: Point = Point {x: 10.3, y: 0.2};

  println!("point: ({},{})",point.x,point.y);
  println!("point 2: ({},{})", another_point.x, another_point.y);
  
  //sorry not gonna do more im lazy

  println!("yay");
}
