// calculator repl impl by Outsider
// Please check for any compilation error!
use std::io;
use std::io::Write;
use std::process::Command;
 
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    None,
}

fn getline(inputPrompt: &str) -> String {
    print!("{}> {}{}{}{}", "\x1b[0;34m", "\x1b[0;0m", "\x1b[0;37m", inputPrompt, "\x1b[0;0m");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input!");
 
    return input.trim_end().to_string();
}
 
fn clearscr() {
  if cfg!(windows) {
    Command::new("cmd")
    .args(&["/c", "cls"])
    .spawn()
    .expect("Failed to clear the screen!")
    .wait()
    .expect("Failed to wait!");
  }
  else {
    Command::new("clear")
     .spawn()
     .expect("Failed to clear the screen!")
     .wait()
     .expect("Failed to wait!");
  }
}
 
fn getEquationMethod(equation: &str) -> &str {
     match equation {
         e if e.contains(&"+") => Operator::Add,
         e if e.contains(&"-") => Operator::Sub,
         e if e.contains(&"*") => Operator::Mul,
         e if e.contains(&"/") => Operator::Div,
     }
}
 
fn doTheFrigginMath(first_operator: String, second_operator: String, equationMethod: &str) -> String {
  // this can error with invalid input
  let raw_op1 = first_operator.parse::<usize>().unwrap();
  let raw_op2 = second_operator.parse::<usize>().unwrap();
  match equationMethod {
     Operator::Add  => raw_op1 + raw_op2,
     Operator::Sub  => raw_op1 - raw_op2,
     Operator::Div  => raw_op1 / raw_op2,
     Operator::Mul  => raw_op1 * raw_op2,
     Operator::None => NONE, 
  }.to_string()
}
 
 
fn main() {
    clearscr();
    loop {
      let mut equation: String = getline("calculator-repl: ");
      let equationMethod: &str = getEquationMethod(&equation);
      let mut sector = 1;
      let mut first_operator = String::new();
      let mut second_operator = String::new();
      for l in equation.chars() {
        if sector == 1 {
          if l.to_string() == equationMethod {
            sector = 2; // move to sector two
          }
          else {
            first_operator.push(l);
          }
        }
        else if sector == 2 {
          second_operator.push(l);
        }
      }
      if sector != 2 {
        println!("invalid equation!");
        std::process::exit(0);
      }
      println!("{}", doTheFrigginMath(first_operator, second_operator, equationMethod));
    }
}
