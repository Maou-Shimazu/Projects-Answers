use port_scanner;
use std::time::SystemTime;

fn main() {
    let (ip, ports) = port_scanner::parse_args();

    let time = SystemTime::now();

    let result = port_scanner::scan(&ip, &ports);

    let elapsed = time.elapsed().unwrap().as_millis();

    println!(
        "Open ports: {}",
        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    println!("Time taken: {} ms", elapsed);
}
