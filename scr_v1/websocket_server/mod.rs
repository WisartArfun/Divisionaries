pub mod ws_connection;

use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::{thread, time};

use log;
use tungstenite;

// use net2::{unix::UnixTcpBuilderExt, TcpBuilder,};

use crate::websocket_server::ws_connection::WSConnection;

use crate::connection::ConnectionServer;

use crate::logic::bucket_server::BaseConnectionHandler;

pub struct WebSocketServer {
    ip: Arc<Mutex<String>>,
    port: Arc<Mutex<String>>,
    running: bool,
    pub handle: Option<thread::JoinHandle<std::io::Result<()>>>,
}

impl ConnectionServer for WebSocketServer {
    fn new(ip: &str, port: &str) -> Self {
        log::info!("new websocket, ip: {}, port: {}", ip, port);
        WebSocketServer {
            ip: Arc::new(Mutex::new(ip.to_string())),
            port: Arc::new(Mutex::new(port.to_string())),
            running: false,
            handle: None,
        }
    }

    // PROB: QUES: generics
    fn start(&mut self, callback: Arc<Mutex<BaseConnectionHandler>>) {
        // QUES: Send? Send unsafe? // QUES: what exactly does 'static do and when to use it
        if self.running {
            return;
        }
        self.running = true;

        let ip = self.ip.clone();
        let port = self.port.clone();
        log::info!(
            "socket started at {}:{}",
            &self.ip.lock().unwrap(),
            &self.port.lock().unwrap()
        );

        let web_socket_handle = thread::spawn(move || -> std::io::Result<()> {
            let server = match TcpListener::bind(format!("{}:{}", ip.lock().unwrap(), port.lock().unwrap())) {
                Ok(res) => res,
                Err(e) => {
                    log::error!("error occured while binding TcpListener: {:?}", e);
                    return Ok(());
                },
            };

            // let server = TcpBuilder::new_v4()?
            //     .reuse_address(true)?
            //     .reuse_port(true)?
            //     .bind(format!("{}:{}", ip.lock().unwrap(), port.lock().unwrap()))?
            //     .listen(42)?;
            for stream in server.incoming() {
                // WARN: handle connection closed // PROB: extremely ugly and not modular
                let stream = stream?;
                stream
                    .set_nonblocking(true)
                    .expect("set_nonblocking call failed");
                log::info!(
                    "A new client connected to {}:{}",
                    ip.lock().unwrap(),
                    port.lock().unwrap()
                );
                thread::sleep(time::Duration::from_millis(5)); // PROB: somehow set_nonblocking needs time => error

                match tungstenite::server::accept(stream) {
                    Ok(websocket) => {
                        log::debug!("calling callback from ConnectionServer");
                        callback
                            .lock()
                            .unwrap()
                            .handle_new_connection(WSConnection::new(websocket));
                    },
                    Err(e) => {
                        log::warn!("an error occured while accepting an incomming connection: {}", e);
                    }
                }
            }

            Ok(())
        });

        self.handle = Some(web_socket_handle);
    }
}
