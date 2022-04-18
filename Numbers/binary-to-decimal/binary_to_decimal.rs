fn dec2bin(dec: usize) -> String {
    (0..((dec as f64).log2() + 1.0).floor() as usize)
        .rev()
        .map(|i| dec / 2usize.pow(i as u32) & 1)
        .map(|n| n.to_string())
        .collect()
}

fn bin2dec(bin: &str) -> usize {
    bin.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| match c {
            '0' => 0,
            '1' => 2usize.pow(i as u32),
            _ => panic!("string did not contain valid binary"),
        })
        .sum()
}

fn main() {
    println!("{}", dec2bin(21));
    println!("{}", bin2dec("10101"))
}
