// calculator repl impl by Outsider
use std::io;
use std::io::Write;
use std::process::Command;
 

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
 
fn getEquationMethod(equation: &str) -> String {
  let ADD = "+".to_string();
  let SUBTRACT = "-".to_string();
  let DIVIDE = "/".to_string();
  let MULTIPLY = "*".to_string();
  let NONE = "".to_string();
  equation.to_owned();
 
  if equation.contains(&ADD) {
    return ADD;
  }
  else if equation.contains(&SUBTRACT) {
    return SUBTRACT;
  }
  else if equation.contains(&DIVIDE) {
    return DIVIDE;
  }
  else if equation.contains(&MULTIPLY) {
    return MULTIPLY;
  }
  else {
    return NONE;
  }
}
 
fn doTheFrigginMath(first_operator: String, second_operator: String, equationMethod: String) -> String {
  let ADD = "+";
  let SUBTRACT = "-";
  let DIVIDE = "/";
  let MULTIPLY = "*";
  let NONE = "";
  let raw_op1 = first_operator.parse::<usize>().unwrap();
  let raw_op2 = second_operator.parse::<usize>().unwrap();
 
  if &equationMethod == ADD {
    return (raw_op1+raw_op2).to_string();
  }
  else if &equationMethod == SUBTRACT {
    return (raw_op1-raw_op2).to_string();
  }
  else if &equationMethod == DIVIDE {
    return (raw_op1/raw_op2).to_string()
  }
  else if &equationMethod == MULTIPLY {
    return (raw_op1*raw_op2).to_string()
  }
  else {
    return NONE.to_string();
  }
}
 
 
fn main() {
    clearscr();
    loop {
      let mut equation: String = getline("calculator-repl: ");
      let equationMethod: String = getEquationMethod(&equation);
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
      let ans: String = doTheFrigginMath(first_operator, second_operator, equationMethod);
      println!("{}", ans);
    }
}
