fn main() {
    for i in 0..101 {
        if i % 3 == 0 {
            print!("fizz ")
        } else if i % 5 == 0 {
            print!("buzz ")
        } else {
            print!("{} ", i)
        };
    }
}
