fn pig_latin(s: &str) -> String {
    fn vowel(c: char) -> bool {
        ['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u'].contains(&c)
    }

    let mut new_s = String::new();
    let mut word = String::new();

    for c in s.chars() {
        if c.is_alphabetic() {
            word.push(c);
        } else {
            if !word.is_empty() {
                let end_i = word
                    .chars()
                    .enumerate()
                    .find(|(_, c)| vowel(*c))
                    .map(|(i, _)| i)
                    .unwrap_or(word.len());

                new_s.push_str(&word[end_i..]);
                if end_i == 0 {
                    new_s.push('y');
                }
                new_s.push_str(&word[..end_i]);
                new_s.push_str("ay");
                word.clear();
            }
            new_s.push(c);
        }
    }

    new_s
}

fn main() {
    let mut input = String::new();
    let stdin = std::io::stdin();

    while let Ok(nbytes) = stdin.read_line(&mut input) {
        if nbytes == 0 {
            break;
        }

        print!("{}", pig_latin(&input));
        input.clear();
    }
}
