/// creates a `WebSocketServer` that can accept connections

use std::{thread, time::Duration};
use std::net::TcpListener;

use log;

use tungstenite;

use super::WSConnection; // QUES: nice way to do it or make mod public and get it from mod???

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
