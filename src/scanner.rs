use std::net::{SocketAddr, TcpStream};
use std::time::Duration;


pub fn create_connection_stream(target_ip: String, target_port: i32)  -> bool {
    let address = format!("{}:{}", target_ip, target_port);
    let socket_addr: SocketAddr = address.parse().expect("unknown address");
    let timeout = Duration::from_millis(50);
    
    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(_) => {
            println!("Found open port -> {}", target_port);
            return true
        }

        Err(_) => {
            return false;
        }
    }
}