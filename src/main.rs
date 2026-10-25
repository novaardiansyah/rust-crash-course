fn main() {
  // ! Scalars

  let age : u32 = 21;
  let num : i32 = -100;
  let temperature : f64 = 20.5;
  let is_student : bool = false;
  let grade : char = 'A';
  
  println!("age: {}, num: {}, temperature: {}, is_student: {}, grade: {}", age, num, temperature, is_student, grade);

  // ! Compoud type

  // ! Tuple
  let person : (&str, u32, bool) = ("Nova", 21, false);
  println!("Name: {}, Age: {}, Is Student: {}", person.0, person.1, person.2);

  // ! Array
  let number : [i32; 5] = [1, 2, 3, 4, 5];
  println!("Number: {}", number[4]);
}
