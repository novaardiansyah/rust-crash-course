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

  // ! While Loop
  let mut n = 3;
  
  while n > 0 {
    println!("{}", n);
    n -= 1;
  }

  println!("Lift Off!");

  // ! For Loop
  let numbers = [1,2,3,4,5];
  for number in numbers {
    println!("{}", number);
  }
}