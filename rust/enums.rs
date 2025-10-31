//idk how enums in rs work compared to that of a c 
//so this should be a bit of an adventure
//however I also know that most of the time when it comes to these tutorials
//they are more for the things which can be used along with the subject 
//than that of the actual subject

enum Cool {
  Success,
  Failiure,
  Half(u8),
}

fn inspect(foo: Cool) {
  match foo {
    Cool::Success => println!("loading successful!"),
    Cool::Failiure => println!("loading unsuccessful. :( "),
    Cool::Half(v) => println!("loading failed at half value {}", v),
  }
}

fn main() {
  let success = Cool::Success;
  let fail = Cool::Failiure;
  let half1 = Cool::Half(1);
  let half2 = Cool::Half(255);
  
  inspect(success);
  inspect(fail);
  inspect(half1);
  inspect(half2);
}
