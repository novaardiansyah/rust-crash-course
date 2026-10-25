fn main() {
  let number : i32 = 6;

  // ! Looping
  let mut counter = 0;

  let result = loop {
    counter += 1;

    println!("Counter : {}", counter);

    if counter == number {
      break counter * 2;
    }
  };

  println!("Result : {}", result);
}