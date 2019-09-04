use tungstenite;
use std::net::TcpStream;

use std::sync::{Arc, Mutex};

use std::thread;

pub struct Client {
    handle: thread::JoinHandle<std::io::Result<()>>, 
    websocket: Arc<Mutex<tungstenite::protocol::WebSocket<TcpStream>>>,
}

impl Client {
    pub fn new(handle: thread::JoinHandle<std::io::Result<()>>, websocket: Arc<Mutex<tungstenite::protocol::WebSocket<TcpStream>>>) -> Client {
        Client{handle, websocket}
    }
}