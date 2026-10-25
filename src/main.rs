fn main() {
  let s1 : String = String::from("Hello");
  let mut s2 : String = s1.clone();

  s2.push('!');

  println!("{}", s1);
  println!("{}", s2);
}