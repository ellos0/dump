fn main() {
   let _logical: bool = true;
   //boolean

   let _a_float: f64 = 1.0;
   let _an_integer = 5i32; //weird suffix notation

   let _default_float = 3.0;
   let _default_integer = 7;

   let mut _inferred = 12; // inferred i64 bc following line
   _inferred = 4294967296i64;

   let mut _mutable = 12; //i32
   _mutable = 21;

   let _array: [i32; 5] = [1,2,3,4,5];
   let _tuple = (5u32, 1u8, true, -5.04f32);
}