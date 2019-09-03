use std::thread;

use tungstenite;
use std::net::TcpStream;

use std::sync::Arc;
use std::sync::Mutex;

pub struct Client {
    // handle: thread::JoinHandle<std::io::Result<()>>, 
    websocket: Arc<Mutex<tungstenite::protocol::WebSocket<TcpStream>>>,
}

impl Client {
    // pub fn new(handle: thread::JoinHandle<std::io::Result<()>>, websocket: Arc<Mutex<tungstenite::protocol::WebSocket<TcpStream>>>) -> Client {
    //     Client{handle, websocket}
    // }

    pub fn new(websocket: Arc<Mutex<tungstenite::protocol::WebSocket<TcpStream>>>) -> Client {
        Client{websocket}
    }
}