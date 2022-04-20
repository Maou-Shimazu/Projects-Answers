use rand::Rng;
use std::env;

fn main() {
    let n: u8 = match env::args().nth(1) {
        Some(n) => n
            .trim()
            .parse()
            .expect("Invalid number of flips, put a positive integer < 256"),
        None => 10, // Default 10
    };

    let mut n_heads = 0u8;
    let mut n_tails = 0u8;

    let result: Vec<&str> = (0..(n as usize))
        .map(|_| match rand::thread_rng().gen_range(0..2) {
            0 => {
                n_heads += 1;
                "Heads"
            }
            1 => {
                n_tails += 1;
                "Tails"
            }
            _ => "Error",
        })
        .collect();

    println!(
        "Result: {:#?}

Total flips: {}
Heads count: {}
Tails count: {}",
        result, n, n_heads, n_tails,
    );
}
