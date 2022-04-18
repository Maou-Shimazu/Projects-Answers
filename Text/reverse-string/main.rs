use std::io::stdin;
use itertools::rev;
fn main() {
  println!("Enter the string you want reversed...");
  let mut str = String::new();
  stdin().read_line(&mut str);
  for i in rev(1..str.len()) {
    println!("{}", str.chars().nth(i - 1).unwrap());
  }
}
