fn main() {
    let original = "Hello World";
    let encrypted = original.shift_right(2);
    let decrypted = &encrypted.shift_left(2);
    println!("{}", encrypted); // Outputs Jgnnq Yqtnf
    println!("{}", decrypted); // Outputs Hello World
}

trait StrExt {
    fn shift_right(&self, amount: usize) -> String;
    fn shift_left(&self, amount: usize) -> String;
}

#[rustfmt::skip]
impl StrExt for str {
    fn shift_right(&self, amount: usize) -> String {
        self.chars()
            .map(|alph| {
                std::char::from_u32(match alph {
                    _ if alph.is_ascii_uppercase() => (65..= 90).cycle().nth(alph as usize - 65 + amount).unwrap(),
                    _ if alph.is_ascii_lowercase() => (97..=122).cycle().nth(alph as usize - 97 + amount).unwrap(),
                    _ => alph as u32,
                })
                .unwrap()
            })
            .collect()
    }

    fn shift_left(&self, amount: usize) -> String {
        self.chars()
            .map(|alph| {
                std::char::from_u32(match alph {
                    _ if alph.is_ascii_uppercase() => (65..= 90).rev().cycle().nth(90  - alph as usize + amount).unwrap(),
                    _ if alph.is_ascii_lowercase() => (97..=122).rev().cycle().nth(122 - alph as usize + amount).unwrap(),
                    _ => alph as u32,
                })
                .unwrap()
            })
            .collect()
    }
}
