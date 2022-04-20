# Port Scanner
Scans provided ports to check if they are open

## Usage
```sh
<executable> <ip.address> <ports>

Examples:

# The flag -rq is a shorthand for the --release and --quiet flags for cargo run
# Checks if the port 3000 of localhost (127.0.0.1) is open or not
cargo run -rq 127.0.0.1 3000

# Multiple ports can be provided separated by a comma, to check at once
cargo run -rq 127.0.0.1 3000,4000,80,443

# If there are whitespaces, use quotations around the ports
cargo run -rq 127.0.0.1 "3000, 4000, 80. 443"

# Ranges of ports can be included using a dash (-) between two ports
# Below it will scan all these ports: 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 3000, 3001, 443
cargo run -rq 127.0.0.1 "75-85, 3000-3001, 443"

# IPV6 is also supported
cargo run -rq ::1 3000
```

## Note
Do not search for a lot of ports at once, it will take a huge amount of time :(
