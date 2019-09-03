use std::{thread, time};
use std::net::TcpListener;
// use std::io::{self, Read};

use tungstenite;

use std::sync::Arc;
use std::sync::Mutex;

use std::boxed::Box;

pub mod client;

// type Callback = fn(client: &Box<client::Client>);
type Callback = fn(client: Arc<Mutex<client::Client>>);


pub struct WebSocket {
    ip: Arc<Mutex<String>>,
    port: Arc<Mutex<String>>,
    running: bool,
    pub handle: Option<thread::JoinHandle<std::io::Result<()>>>,
    clients: Arc<Mutex<Vec<client::Client>>>,
    callback: Callback,
}

impl WebSocket {
    pub fn new<S>(ip: S, port: S, callback: Callback) -> WebSocket where S: Into<String> {
        WebSocket{ip: Arc::new(Mutex::new(ip.into())), port: Arc::new(Mutex::new(port.into())), running: false, handle: None, clients: Arc::new(Mutex::new(Vec::new())), callback}
    }

    pub fn start(&mut self) {
        if self.running {return;}
        self.running = true;

        let ip = self.ip.clone();
        let port = self.port.clone();

        let clients = self.clients.clone();
        let web_socket_handle = thread::spawn(move || -> std::io::Result<()> {
            let server = TcpListener::bind(format!("{}:{}", ip.lock().unwrap(), port.lock().unwrap())).unwrap();
            for stream in server.incoming() { // handle connection closed
                let stream = stream?;
                stream.set_nonblocking(true).expect("set_nonblocking call failed");
                
                println!("A new client connected.");
                thread::sleep(time::Duration::from_millis(5)); // somehow set_nonblocking needs time => error

                let websocket = Arc::new(Mutex::new(tungstenite::server::accept(stream).unwrap()));

                let websocket_clone = websocket.clone();

                clients.lock().unwrap().push(client::Client::new(websocket));
                (self.callback)(Arc::new(Mutex::new(clients.lock().unwrap()[0])));

                // let client: &'static Box<client::Client> = &Box::new(client::Client::new(websocket));
                // (self.callback)(client);
                // (self.callback)(Box::new(client::Client::new(websocket)));

                // let client_handle = thread::spawn (move || -> std::io::Result<()> { // add keep-alive stuff ?
                //     let websocket = websocket_clone;
                    
                //     // let mut web_clone = websocket.clone();
                //     // thread::spawn(move || -> std::io::Result<()> {
                //     //     loop {
                //     //         thread::sleep(time::Duration::from_millis(1000));
                //     //         web_clone.lock().unwrap().write_message(tungstenite::Message::Text("21020311".to_string())).unwrap();
                //     //     }

                //     //     Ok(())
                //     // });

                //     loop {
                //         match websocket.lock().unwrap().read_message() {
                //             Ok(msg) => {
                //                 if msg.is_binary() || msg.is_text() {
                //                     println!("received message from client: {:?}", msg);
                //                 }
                //                 break;
                //             },
                //             // correct error handling
                //             Err(ref _e) => { //if e.kind == io::ErrorKind::WouldBlock => {
                //                 break;
                //             }
                //             // Err(e) => panic!("encountered IO error: {}", e),
                //         };
                //     }

                //     Ok(())
                // });

                // let client = client::Client::new(client_handle, websocket);
                // (self.callback)(client);
                // clients.lock().unwrap().push(client);
            }

            Ok(())
        });

        self.handle = Some(web_socket_handle);
    }
}