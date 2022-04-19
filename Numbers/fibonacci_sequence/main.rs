struct Fibo {
    a: u64,
    b: u64,
}

impl Iterator for Fibo {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let next = self.a;
        (self.a, self.b) = (self.b, self.a + self.b);
        Some(next)
    }
}

fn main() {
    use std::io::{self, Write};

    let mut input = String::new();
    print!("Enter a cap: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let cap = input.trim().parse().unwrap();

    Fibo { a: 0, b: 1 }
        .take_while(|n| *n <= cap)
        .for_each(|n| println!("{}", n));
}
