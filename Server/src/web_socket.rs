use std::{thread, time};
use std::net::TcpListener;

use crate::logic;
use crate::logic::game::SecureAdd;

use tungstenite;

use std::sync::{Arc, Mutex};

pub struct WebSocket {
    ip: Arc<Mutex<String>>,
    port: Arc<Mutex<String>>,
    running: bool,
    pub handle: Option<thread::JoinHandle<std::io::Result<()>>>,
}

impl WebSocket { // register callbacks for receive???
    pub fn new<S: Into<String>>(ip: S, port: S) -> WebSocket {
        WebSocket{ip: Arc::new(Mutex::new(ip.into())), port: Arc::new(Mutex::new(port.into())), running: false, handle: None}
    }

    pub fn start(&mut self, game: Arc<Mutex<logic::game::SecureList>>) {
        if self.running {return;}
        self.running = true;

        let ip = self.ip.clone();
        let port = self.port.clone();

        let web_socket_handle = thread::spawn(move || -> std::io::Result<()> {
            let server = TcpListener::bind(format!("{}:{}", ip.lock().unwrap(), port.lock().unwrap())).unwrap();
            for stream in server.incoming() { // handle connection closed
                let stream = stream?;
                stream.set_nonblocking(true).expect("set_nonblocking call failed");
                println!("A new client connected.");
                
                thread::sleep(time::Duration::from_millis(5)); // somehow set_nonblocking needs time => error

                let websocket = Arc::new(Mutex::new(tungstenite::server::accept(stream).unwrap()));

                let client = logic::client::Client::new(websocket);
                (*game.lock().unwrap()).add(client); // return instead of add
            }

            Ok(())
        });

        self.handle = Some(web_socket_handle);
    }
}