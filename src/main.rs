fn main() {
  for i in 1..5 {
    println!("{}", i);
  }

  println!("Lift off!");

  let scores : [i32; 3] = [10, 20, 30];

  for score in scores {
    println!("{}", score);
  }
}