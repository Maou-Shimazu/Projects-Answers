use std::collections::HashMap;

fn count_chars(s: &str, charset: &[char]) -> HashMap<char, usize> {
    charset
        .iter()
        .map(|&v| (v, s.chars().filter(|&c| c == v).count()))
        .collect()
}

fn main() {
    use std::io::Read;

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    input.make_ascii_lowercase();

    let counts = count_chars(&input, &['a', 'e', 'i', 'o', 'u']);
    counts.into_iter().for_each(|(v, n)| println!("{v}: {n}"));
}
