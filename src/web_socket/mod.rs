//! handles websocket connections and servers

pub use connection::WSConnection;
pub use server::WebSocketServer;

/// handles a connection to a `WebSocketServer`
mod connection {
    use std::net::TcpStream;
    use std::sync::{Arc, Mutex};

    use tungstenite;

    /// stores and manipulates a WebSocket
    /// 
    /// # Variables
    /// 
    /// * ws_conn: `Arc<Mutex<tungstenite::protocol::WebSocket<TcpStream>>>` - the connection
    pub struct WSConnection {
        ws_conn: Arc<Mutex<tungstenite::protocol::WebSocket<TcpStream>>>,
    }

    impl WSConnection {
        /// creates a new `WSConnection`
        /// 
        /// # Arguments
        /// 
        /// * ws_conn: `tungstenite::protocol::WebSocket<TcpStream>` - the connection
        /// 
        /// # Returns
        /// 
        /// * an instance of `WSConnection` that handles `ws_conn`
        pub fn new(ws_conn: tungstenite::protocol::WebSocket<TcpStream>) -> Self {
            log::info!("creating new WSConnection");
            Self {
                ws_conn: Arc::new(Mutex::new(ws_conn))
            }
        }
        
        /// closes the connection
        pub fn close(&mut self) { // WARN: do this with a trait
            log::info!("closing WSConnection");
            self.ws_conn.lock().unwrap().close(None).unwrap(); // WARN: unsafe unwrap
        }

        /// sends a message over the connection if possible, otherwise returns an `Error`
        /// 
        /// # Arguments
        /// 
        /// * message: `Vec<u8>` - the message to be sent over the connection
        /// 
        /// # Returns
        /// 
        /// * outcome: `Result<(), tungstenite::error::Error>` - wheter sending was successfull
        pub fn send(&mut self, message: Vec<u8>) -> Result<(), tungstenite::error::Error> { // QUES: return other error type
            log::debug!("sending message over WSConnection");
            self.ws_conn.lock().unwrap().write_message(tungstenite::Message::Binary(message))?;
            Ok(())
        }

        /// tries to receive a message over the connection, but does not block if there is no message to receive
        /// 
        /// # Returns
        /// 
        /// * message: `Result<Option<Vec<u8>>, tungstenite::error::Error>` - an `Option` with the message if one was received, if an error occured while receiving a message, an `tungstenite::error::Error` is returned
        pub fn try_recv(&mut self) -> Result<Option<Vec<u8>>, tungstenite::error::Error> {
            match self.ws_conn.lock().unwrap().read_message() {
                Ok(msg) => {
                    log::debug!("received message from WSConnection.websocket");
                    if msg.is_binary() || msg.is_text() {
                        return Ok(Some(msg.into_data()));
                    }
                },
                // QUEST: correct error handling
                Err(e) => {
                    if let tungstenite::error::Error::Io(err) = &e {
                        if err.kind() == std::io::ErrorKind::WouldBlock {
                            return Ok(None);
                        };
                    }
                    log::warn!("an error occured while receiving a message from WSConnection: {}", &e);
                    return Err(e);
                }
            };

            Ok(None)
        }
    }
}

/// creates a `WebSocketServer` that can accept connections
pub mod server {
    use std::{thread, time::Duration};
    use std::net::TcpListener;

    use log;

    use tungstenite;

    use super::connection::WSConnection;

    /// a simple `WebSocketServer` that will handle all incoming connections with a closure
    /// 
    /// # Type Parameters
    /// 
    /// * `'a` - the lifetime for ip and port
    /// 
    /// # Variables
    /// 
    /// * ip: `&'a str` - the ip of the `WebSocketServer`
    /// * port: `&'a str` - the port of the `WebScoketServer`
    /// * running: `bool` - whether the `WebSocketServer` is running or not
    pub struct WebSocketServer<'a> {
        ip: &'a str,
        port: &'a str,
        running: bool,
    }

    impl<'a> WebSocketServer<'a> {
        /// creates a new `WebSocketServer` instance
        /// 
        /// # Arguments
        /// 
        /// * ip: `&'a str` - the ip of the `WebSocketServer`
        /// * port: `&'a str` - the port of the `WebSocketServer`
        /// 
        /// # Returns
        /// 
        /// ws_instance: `WebSocketServer` - an instance of a `WebScoketServer`
        pub fn new(ip: &'a str, port: &'a str) -> Self {
            Self {
                ip,
                port,
                running: false,
            }
        }
        
        // QUES: when to use std::io::Result and when Result with dyn std::error::Error
        // WARN: 'static => not allowed to have borrowed values without 'static

        /// Takes a closure that handles the connections and starts the `WebScoketServer`.
        /// If the instance was already running, None is returned.
        /// 
        /// # Type Parameters
        /// 
        /// * F: `FnMut(WSConnection) -> () + Send + 'static` - type for the callback
        /// 
        /// # Arguments
        /// 
        /// * callback: `F` - the callback to handle connections
        /// 
        /// # Returns
        /// 
        /// * handle: `Option<thread::JoinHandle<std::io::Result<()>>>` - returns a handle if the server was not already running
        pub fn start<F: FnMut(WSConnection) -> () + Send + 'static>(&mut self, mut callback: F) -> Option<thread::JoinHandle<std::io::Result<()>>> {
            if self.running {
                log::warn!("trying to start WebSocketServer, although it is already running");
                return None;
            }
            self.running = true;

            log::debug!("starting websocket at: {}:{}", self.ip, self.port);
            let ip = self.ip.to_string();
            let port = self.port.to_string();

            Some(thread::spawn(move || -> std::io::Result<()> {
                let server =TcpListener::bind(format!("{}:{}", &ip, &port))?; // should return errro in case of fail

                for stream in server.incoming() {
                    log::debug!("a new client is connecting to a WebSocketServer at {}:{}", &ip, &port);
                    let stream = stream?;
                    stream.set_nonblocking(true)?;
                    thread::sleep(Duration::from_millis(5));
                    
                    match tungstenite::server::accept(stream) {
                        Ok(socket) => {
                            log::debug!("calling callbock from WebSocketServer");
                            callback(WSConnection::new(socket));
                        },
                        Err(e) => log::warn!("an error occured while accepting an incoming connection: {}", e), // WARN: server will continue even if an error occures
                    }
                }

                Ok(())
            }))
        }
    }
}
