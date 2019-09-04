use tungstenite;
use std::net::TcpStream;

use std::sync::{Arc, Mutex};

use std::thread;

pub struct Client {
    // handle: thread::JoinHandle<std::io::Result<()>>, 
    websocket: Arc<Mutex<tungstenite::protocol::WebSocket<TcpStream>>>, // make private again
}

impl Client {
    // pub fn new(handle: thread::JoinHandle<std::io::Result<()>>, websocket: Arc<Mutex<tungstenite::protocol::WebSocket<TcpStream>>>) -> Client {
    //     Client{handle, websocket}
    // }

    pub fn new(websocket: Arc<Mutex<tungstenite::protocol::WebSocket<TcpStream>>>) -> Client {
        Client{websocket}
    }

    pub fn send(&self, message: &str) { // 21020311
        self.websocket.lock().unwrap().write_message(tungstenite::Message::Text(message.to_string())).unwrap();
    }

    pub fn try_recv(&self) -> Option<String> {
        match self.websocket.lock().unwrap().read_message() {
            Ok(msg) => {
                println!("hello there");
                if msg.is_binary() || msg.is_text() {
                    println!("received message from client: {:?}", msg);
                    return Some(msg.to_string());
                }
            },
            // correct error handling
            Err(ref _e) => {} //if e.kind == io::ErrorKind::WouldBlock => {
            // Err(e) => panic!("encountered IO error: {}", e),
        };

        None
    }
}