use std::net::TcpListener;

pub fn get_available_port(ip: &str) -> Option<u16> {
    (8001..9000)
        .find(|port| port_is_available(ip, *port))
}

fn port_is_available(ip: &str, port: u16) -> bool {
    match TcpListener::bind((ip, port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}