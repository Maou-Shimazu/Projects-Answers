struct Fibo {
    cap: u64,
    a: u64,
    b: u64,
}

impl Fibo {
    fn new(cap: u64) -> Self {
        Self { cap, a: 0, b: 1 }
    }
}

impl Iterator for Fibo {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        (self.a < self.cap).then(|| {
            let next = self.a;
            let sum = self.a + self.b;
            (self.a, self.b) = (self.b, sum);
            next
        })
    }
}

fn main() {
    use std::io::{self, Write};

    let mut input = String::new();
    print!("Enter a cap: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let cap = input.trim().parse().unwrap();

    Fibo::new(cap).for_each(|n| println!("{}", n));
}
