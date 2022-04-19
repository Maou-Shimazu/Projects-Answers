fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input
        .chars()
        .zip(input.chars().rev())
        .all(|(c1, c2)| c1 == c2)
    {
        println!("That is a palindrome!");
    } else {
        println!("That is not a palindrome.");
    }
}
