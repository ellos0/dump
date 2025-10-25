use std::mem;

fn analyze_slice(slice: &[i32]) {
   println!("first element of the slice: {}", slice[0]);
   println!("the slice has {} elements", slice.len());
}

fn main() {
   let xs: [i32; 5] = [1,2,3,4,5];
   // fixed size array
   let ys: [i32; 500] = [0; 500];
   // elements can be initialized to the same value
   println!("first {}", xs[0]);
   println!("second {}", xs[1]);

   println!("length {}", xs.len());
   println!("array bytes: {}", mem::size_of_val(&xs));

   println!("slice time");
   analyze_slice(&xs);

   //slices point to a section of an array
   //u can use [start..end]
   println!("borrow a section");
   analyze_slice(&ys[1 .. 4]);

   let empty_array: [u32; 0] = [];
   assert_eq!(&empty_array, &[]);
   assert_eq!(&empty_array, &[][..]); // same but verbose

   println!("done!");
}