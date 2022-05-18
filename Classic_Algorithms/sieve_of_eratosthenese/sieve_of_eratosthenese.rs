// Took a unique approach to this algorithm, hope you like it
// New to Rust btw so don't bully me lmao

fn main() {
  let mut prime: Vec<bool> = vec![];

  for i in 1..101 {
    if (i % 2 == 0) && (i >= 2*2) {
      prime.push(true);
    } else if (i % 3 == 0) && (i >= 3*3) {
      prime.push(true);
    } else if (i % 5 == 0) && (i >= 5*5) {
      prime.push(true);
    } else {
      prime.push(false);
    }
  }

  for i in 0..100 {
    if prime[i] == true {
      println!("{}", i);
    }
  }
}
