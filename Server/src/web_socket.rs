use std::thread;
use std::net::TcpListener;
use tungstenite::server::accept;

use std::sync::Arc;
use std::sync::Mutex;

pub struct WebSocket {
    ip: Arc<Mutex<String>>,
    port: Arc<Mutex<String>>,
    running: bool,
    pub handle: Option<thread::JoinHandle<std::io::Result<()>>>,
}

impl WebSocket {
    pub fn new<S>(ip: S, port: S) -> WebSocket where S: Into<String> {
        WebSocket{ip: Arc::new(Mutex::new(ip.into())), port: Arc::new(Mutex::new(port.into())), running: false, handle: None}
    }

    pub fn start(&mut self) {
        if self.running {return;}
        self.running = true;

        let ip = self.ip.clone();
        let port = self.port.clone();

        let handle = thread::spawn(move || -> std::io::Result<()> {
            let server = TcpListener::bind(format!("{}:{}", ip.lock().unwrap(), port.lock().unwrap())).unwrap();
            for stream in server.incoming() { // handle connection closed
                println!("hello there");
                thread::spawn (move || { // add keep-alive stuff ?
                    // let mut websocket = accept(stream.unwrap(), None).unwrap();
                    let mut websocket = accept(stream.unwrap()).unwrap();
                    loop {
                        let msg = websocket.read_message().unwrap();

                        // no ping/pong
                        if msg.is_binary() || msg.is_text() {
                            // websocket.write_message(msg).unwrap();
                            websocket.write_message(tungstenite::Message::Text("21020311".to_string())).unwrap(); // nicer
                        }
                    }
                });
            }

            Ok(())
        });

        self.handle = Some(handle);
    }
}