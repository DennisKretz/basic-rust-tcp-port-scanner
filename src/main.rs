mod scanner;

use std::io;

const MAX_PORT_RANGE: i32 = 65535;

struct PortScanner {
    target_ip: String,
    port_range: i32,
    open_ports: Vec<String>
}

impl PortScanner {
    fn start(&mut self) {
        match is_valid_ip_format(&self.target_ip) {
            true => println!("IP FORMAT IS VALID"),
            false => panic!("IP FORMAT IS INVALID")
        }

        match is_valid_port_range(self.port_range) {
            true => println!("PORT RANGE IS VALID"),
            false => panic!("PORT RANGE IS INVALID {}", self.port_range)
        }

        let mut i: i32 = 0;

        while i <= self.port_range {
            println!("PROBING PORT -> {}", i);

            match scanner::create_connection_stream(self.target_ip.to_string(), i) {
                true => self.open_ports.push(i.to_string()),
                false => {}
            }
            i = i + 1;
        }

        println!("OPEN PORTS -> {:?}", self.open_ports);
    }

    fn set_target_ip(&mut self) {
        let mut input: String = String::new();

        println!("ENTER IP:");

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading the input");
        
        self.target_ip = input.trim().to_string();
    }

    fn set_port_range(&mut self) {
        let mut input: String = String::new();

        println!("ENTER PORT RANGE:");

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading the input");

        self.port_range = input.trim().parse::<i32>().unwrap();
    }
}

fn main() {
    let mut port_scanner = PortScanner {
        target_ip: String::new(),
        port_range: MAX_PORT_RANGE,
        open_ports: Vec::new()
    };

    port_scanner.set_target_ip();
    port_scanner.set_port_range();
    port_scanner.start();
}

fn is_valid_port_range(port_range: i32) -> bool {
    return (port_range <= MAX_PORT_RANGE) && (0 < port_range);
}

fn is_valid_ip_format(target_ip: &str) -> bool {
    let ip_octets: Vec<&str> = target_ip.trim().split(".").collect();

    if ip_octets.len() != 4 { return false; }

    for octet in ip_octets {
        if let Ok(_num) = octet.parse::<u8>() {
            continue;
        } else {
            return false;
        }
    }

    return true;
}