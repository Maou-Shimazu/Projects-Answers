use std::collections::HashMap;

fn prime_fact(mut num: usize) -> HashMap<usize, usize> {
    let mut known_primes = Vec::new();
    struct LazyPrimes<'a> {
        known: &'a mut Vec<usize>,
        i: usize,
    }
    impl<'a> LazyPrimes<'a> {
        fn new(known: &'a mut Vec<usize>) -> Self {
            Self { known, i: 0 }
        }
    }
    impl Iterator for LazyPrimes<'_> {
        type Item = usize;
        fn next(&mut self) -> Option<usize> {
            if self.i == self.known.len() {
                let next_prime = (self.known.last().copied().unwrap_or(1) + 1..)
                    .find(|n| self.known.iter().copied().all(|p| n % p != 0))
                    .unwrap();
                self.known.push(next_prime);
            }
            self.i += 1;
            Some(self.known[self.i - 1])
        }
    }

    let mut factors = HashMap::new();

    while num != 1 {
        let next_factor = LazyPrimes::new(&mut known_primes)
            .find(|p| num % p == 0)
            .unwrap();
        *factors.entry(next_factor).or_default() += 1;
        num /= next_factor;
    }

    factors
}

fn main() {
    use std::io::{self, Write};

    let mut input = String::new();
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let num = input.trim().parse().unwrap();
    let fact = prime_fact(num);

    println!(
        "{num} = {}",
        fact.into_iter()
            .map(|(p, n)| match n {
                1 => p.to_string(),
                _ => format!("{p}^{n}"),
            })
            .collect::<Vec<_>>()
            .join(" * ")
    );
}
