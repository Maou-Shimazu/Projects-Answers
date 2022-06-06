use std::io::stdin;

fn main() {
    println!("Please enter the card number...");
    let mut card = String::new();
    stdin().read_line(&mut card).ok();

    let mut sum: u32 = 0;
  
    for num in card.trim().chars().rev() {
      let u_num = num.to_digit(10).expect("Please enter a valid number!");
      let value = u_num * 2;
      if value > 9 {
        sum += value - 9;
      } else {
        sum += value;
      }
    }

    if sum % 10 == 0 {
      println!("Valid card number");
    } else {
      println!("Invalid card number");
    }
}
