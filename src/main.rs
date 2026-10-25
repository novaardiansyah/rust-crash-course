fn main() {
  great("Nova Ardiansyah");
  println!("Result: {}", add(10, 20));
}

fn great(fullname: &str) {
  println!("Hello, {}", fullname);
}

fn add(a: i32, b: i32) -> i32 {
  a + b // ! without return keyword and semicolon is an expression
}