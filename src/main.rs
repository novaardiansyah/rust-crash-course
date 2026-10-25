fn main() {
  let number : i32 = 5;
  
  let label : &str = if number % 2 == 0 {"Even"} else {"Odd"};

  println!("label : {}", label);
}