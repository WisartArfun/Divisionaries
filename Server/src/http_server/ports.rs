use std::net::TcpListener;

use std::sync::atomic::{AtomicU16, Ordering};

static START_IP: AtomicU16 = AtomicU16::new(8001);

pub fn get_available_port(ip: &str) -> Option<u16> {
    (START_IP.fetch_add(1, Ordering::SeqCst)..9000)
        .find(|port| port_is_available(ip, *port))
}

fn port_is_available(ip: &str, port: u16) -> bool {
    match TcpListener::bind((ip, port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}