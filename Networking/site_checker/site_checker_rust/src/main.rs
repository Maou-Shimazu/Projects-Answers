use text_io::*;
use std::io::{stdout, Write};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    print!("Url you want to check: ");
    stdout().flush().ok();
    let url: String = read!();
    print!("Interval (seconds) to check the site in: ");
    stdout().flush().ok();
    let mut interval: u64 = read!();
    
    interval = interval * 1000;

    let response = reqwest::blocking::get(&url).unwrap();
    loop {
        match response.status() {
            reqwest::StatusCode::OK => {
                println!("Success! {} is online.", url);
            },
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("This site is unauthorized.");
                break;
            },
            _ => {
                println!("The site is down.");
                for i in 1..6 {
                    println!("retrying {}", i);
                    sleep(Duration::from_millis(interval));
                }
                println!("Request failed.");
                break;
            },
        };
        sleep(Duration::from_millis(interval));
    }
    
}
