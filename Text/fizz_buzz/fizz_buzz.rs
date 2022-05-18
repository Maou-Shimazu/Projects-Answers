fn main() {
    for num in 1..101 {
      if (num % 3 == 0) && (num % 5 == 0) {
        println!("FizzBuzz");
      } else if num % 3 == 0 {
        println!("Fizz");
      } else if num % 5 == 0 {
        println!("Buzz");
      } else {
        println!("{}", num);
      }
    }
}
