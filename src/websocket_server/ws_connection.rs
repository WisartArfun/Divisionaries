use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::io;

use tungstenite;
use log;

use crate::connection::Connection;

pub struct WSConnection { // PROB: more generics
    ws_conn: Arc<Mutex<tungstenite::protocol::WebSocket<TcpStream>>>, // QUES: more generics???
}

impl Connection for WSConnection {
    fn send(&mut self, message: Vec<u8>) {
        log::debug!("send message over WSConnection"); // QUES: also save message?
        self.ws_conn.lock().unwrap().write_message(tungstenite::Message::Binary(message)).unwrap();
    }

    fn try_recv(&mut self) -> Option<Vec<u8>> {
        match self.ws_conn.lock().unwrap().read_message() {
            Ok(msg) => {
                log::debug!("received message from WSConnection.websocket");
                if msg.is_binary() || msg.is_text() {
                    return Some(msg.into_data());
                }
            },
            // QUEST: correct error handling
            Err(ref e) => {
                if let tungstenite::error::Error::Io(err) = e {
                    if err.kind() == io::ErrorKind::WouldBlock {
                        return None;
                    };
                }
                log::warn!("an error occured while receiving a message from WSConnection: {}", e);
            }
        };

        None
    }
}

impl WSConnection {
    pub fn new(ws_conn: tungstenite::protocol::WebSocket<TcpStream>) -> WSConnection {
        log::info!("created new WSConnection");
        WSConnection{ws_conn: Arc::new(Mutex::new(ws_conn))}
    }

    pub fn close(&mut self) { // WARN: do this with a trait
        log::info!("closing WSConnection");
        self.ws_conn.lock().unwrap().close(None);
    }
}