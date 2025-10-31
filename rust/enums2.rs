enum Operators {
  Add,
  Subtract,
}

impl Operators {
  fn run(&self,x:i32,y:i32) -> i32 {
    match self {
      Self::Add => x + y,
      Self::Subtract => x - y,
    }
  }
}

fn main() {
  let add = Operators::Add;
  let sub = Operators::Subtract;

  println!("addition result: {}", add.run(2,3));
  println!("subtraction result: {}", sub.run(9,5));
}
