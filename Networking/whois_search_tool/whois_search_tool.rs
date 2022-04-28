use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process;

fn main() {
    // This section parses command line args
    let mut args = env::args();
    args.next();
    let query = args
        .next()
        .error("Error: No query was provided. Provide a query in the command line argument.");

    // Create connection to IANA's whois server
    let mut stream = TcpStream::connect("whois.iana.org:43")
        .error("Error: Could not connect to the whois server.");

    // request body following the WHOIS protocol
    let request = format!("{}\r\n", query);

    // Send request
    stream
        .write_all(&mut request.as_bytes())
        .error("Error: Request failed due to network error.");

    // Read response
    let mut final_buffer = Vec::with_capacity(2048 + 512);
    stream
        .read_to_end(&mut final_buffer)
        .error("Error: Failed to read the response.");

    let response = String::from_utf8(final_buffer).error("Error: Invalid response was recieved.");

    // Extract the whois server from IANA's response
    let whois_server = response
        .lines()
        .find(|line| line.contains("whois:"))
        .error("Error: No whois server found.")
        .split(':')
        .nth(1)
        .error("Error: No whois server found.")
        .trim();

    // Create connection to the extracted whois server
    // and follow the previous steps
    let mut stream = TcpStream::connect(whois_server.to_owned() + ":43")
        .error("Error: Could not connect to the whois server.");

    let request = format!("{}\r\n", query);

    stream
        .write_all(&mut request.as_bytes())
        .error("Error: Request failed due to network error.");

    let mut final_buffer = Vec::with_capacity(4096);
    stream
        .read_to_end(&mut final_buffer)
        .error("Error: Failed to read the response.");

    let response = String::from_utf8(final_buffer).error("Error: Invalid response was recieved.");

    println!("{}", response);
}

// Helper trait, dont mind :|
trait PlainError<T> {
    fn error(self, msg: &str) -> T;
}

impl<T, E> PlainError<T> for Result<T, E> {
    fn error(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(_) => {
                eprintln!("{}", msg);
                process::exit(1);
            }
        }
    }
}
impl<T> PlainError<T> for Option<T> {
    fn error(self, msg: &str) -> T {
        match self {
            Some(t) => t,
            None => {
                eprintln!("{}", msg);
                process::exit(1);
            }
        }
    }
}

