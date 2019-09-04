use std::{thread, time};
use std::net::TcpListener;

use crate::logic;

use tungstenite;

use std::sync::{Arc, Mutex};

// use super::logic;
// use crate::logic;

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

    pub fn start(&mut self, game: Arc<Mutex<SecureList>>) {
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

                let websocket_clone = websocket.clone();
                let client_handle = thread::spawn (move || -> std::io::Result<()> { // add keep-alive stuff ?
                    let websocket = websocket_clone;
                    
                    // let mut web_clone = websocket.clone();
                    // thread::spawn(move || -> std::io::Result<()> {
                    //     loop {
                    //         thread::sleep(time::Duration::from_millis(1000));
                    //         web_clone.lock().unwrap().write_message(tungstenite::Message::Text("21020311".to_string())).unwrap();
                    //     }

                    //     Ok(())
                    // });

                    loop {
                        match websocket.lock().unwrap().read_message() {
                            Ok(msg) => {
                                if msg.is_binary() || msg.is_text() {
                                    println!("received message from client: {:?}", msg);
                                }
                                break;
                            },
                            // correct error handling
                            Err(ref _e) => { //if e.kind == io::ErrorKind::WouldBlock => {
                                break;
                            }
                            // Err(e) => panic!("encountered IO error: {}", e),
                        };
                    }

                    Ok(())
                });

                let client = client::Client::new(client_handle, websocket);
                // let client = client::Client::new(websocket);
                game.lock().unwrap().add(client);
            }

            Ok(())
        });

        self.handle = Some(web_socket_handle);
    }
}