use std::{
    env,
    net::{IpAddr, SocketAddr, TcpStream},
    process,
};

pub fn scan(ip: &IpAddr, ports: &[u16]) -> Vec<u16> {
    let mut out = Vec::with_capacity(10);

    for &port in ports {
        let socket_addr = SocketAddr::new(*ip, port);
        match TcpStream::connect(&socket_addr) {
            Ok(_) => out.push(port),
            Err(_) => (),
        }
    }

    out
}

pub fn parse_args() -> (IpAddr, Vec<u16>) {
    let mut args_iter = env::args();
    args_iter.next();

    let ip_arg = match args_iter.next() {
        Some(arg) => arg,
        None => {
            eprintln!(
                "Error   : This program expects 2 command line arguments, none provided.\n\n{}",
                HELP_TEXT
            );
            process::exit(1);
        }
    };

    let ports_arg = match args_iter.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Error   : Ports are not provided. Ports are expected as the second command line argument.\n\n{}", HELP_TEXT);
            process::exit(1);
        }
    };

    let ports: Vec<u16> = ports_arg
        .split(',')
        .map(|p| {
            let mut peter = p.split('-');
            let start_arg = match peter.next() {
                Some(pp) => pp.trim(),
                None => {
                    eprintln!("Error: This should not have happened");
                    process::exit(1);
                }
            };
            let end_arg = match peter.next() {
                Some(pp) => pp.trim(),
                None => start_arg,
            };
            let start: u16 = match start_arg.parse() {
                Ok(pp) => pp,
                Err(_) => {
                    eprintln!("{} is not a valid port.", start_arg);
                    process::exit(1);
                }
            };
            let end: u16 = match end_arg.parse() {
                Ok(pp) => pp,
                Err(_) => {
                    eprintln!("{} is not a valid port.", end_arg);
                    process::exit(1);
                }
            };
            let r = std::ops::Range {
                start: start as usize,
                end: (end + 1) as usize,
            };
            r.map(|x| x as u16)
        })
        .flatten()
        .collect();

    let ip = match ip_arg.parse() {
        Ok(the_ip) => the_ip,
        Err(_) => {
            eprintln!("Error: \"{}\" is not a valid IP address.", ip_arg);
            process::exit(1);
        }
    };

    (ip, ports)
}

const HELP_TEXT: &str = "Port scanner
Scans provided ports to check if they are open.

Usage   : <executable> <ip.address> <ports>
Example : cargo run -rq -- 127.0.0.1 \"3000, 8000\"

Different ports should be separated by a comma (,). Example: 3000,4000,80,443
If there are whitespaces between them, use quotations (\" \") around them. Example: \"3000, 4000, 80, 443\"

Ranges of ports can be included using a dash (-) between two ports. Example: \"75-85, 3000-3010, 443\"

Note: Do not search for a lot of ports at once, it will take a huge amount of time.";
