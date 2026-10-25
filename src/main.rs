const MAX_POINTS: u32 = 100_000;

fn main() {
  let mut x = 6;
  println!("x is {}", x);

  x = 7;
  println!("x is {}", x);

  let y = 5;
  println!("y is {}", y);

  // ! shadowing
  let y = y + 2;
  println!("y is {}", y);

  println!("max points is {}", MAX_POINTS);
}
