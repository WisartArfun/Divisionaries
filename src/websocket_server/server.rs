use std::{thread, time};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use tungstenite;
use log;

use crate::websocket_server::ws_connection::WSConnection;
use crate::connection::HandleNewConnection;

use crate::connection::ConnectionServer;

pub struct WebSocketServer {
    ip: Arc<Mutex<String>>,
    port: Arc<Mutex<String>>,
    running: bool,
    pub handle: Option<thread::JoinHandle<std::io::Result<()>>>,
}

impl ConnectionServer for WebSocketServer {
    fn new(ip: &str, port: &str) -> Self {
        log::info!("new websocket, ip: {}, port: {}", ip, port);
        WebSocketServer{ip: Arc::new(Mutex::new(ip.to_string())), port: Arc::new(Mutex::new(port.to_string())), running: false, handle: None}
    }

    // PROB: QUES: generics
    fn start<T: HandleNewConnection + Send + 'static>(&mut self, callback: Arc<Mutex<T>>) { // QUES: Send? Send unsafe? // QUES: what exactly does 'static do and when to use it
    // fn start(&mut self, callback: Arc<Mutex<dyn HandleNewConnection>>) { // PROB: the the `handle_new_connection` method cannot be invoked on a trait object
        if self.running {return;}
        self.running = true;

        let ip = self.ip.clone();
        let port = self.port.clone();
        log::info!("socket started at {}:{}", &self.ip.lock().unwrap(), &self.port.lock().unwrap());

        let web_socket_handle = thread::spawn(move || -> std::io::Result<()> {
            let server = TcpListener::bind(format!("{}:{}", ip.lock().unwrap(), port.lock().unwrap())).unwrap();
            for stream in server.incoming() { // WARN: handle connection closed // PROB: extremely ugly and not modular
                let stream = stream?;
                stream.set_nonblocking(true).expect("set_nonblocking call failed");
                log::info!("A new client connected to {}:{}", ip.lock().unwrap(), port.lock().unwrap());
                
                thread::sleep(time::Duration::from_millis(5)); // PROB: somehow set_nonblocking needs time => error

                let websocket = tungstenite::server::accept(stream).unwrap(); // do this generic

                callback.lock().unwrap().handle_new_connection(WSConnection::new(websocket));
            }

            Ok(())
        });

        self.handle = Some(web_socket_handle);
    }
}