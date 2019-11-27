//! creates a server around a bucket

use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

use log;

use crate::web_socket::{WSConnection, WebSocketServer};

use super::types::{Bucket, BucketClient, BucketMessage};

/// handles connections
///
/// # Variables
///
/// * `pub` connections: `HashMap<u64, Arc<Mutex<BucketClient>>>` - represents all connections
/// * current_id: `u64` - lowest unused id
struct ConnectionHandler {
    pub connections: HashMap<u64, Arc<Mutex<BucketClient>>>,
    current_id: u64,
}

impl ConnectionHandler {
    /// creates a new ConnectionHandler
    ///
    /// # Returns
    ///
    /// * instance: `ConnectionHandler` - a new `ConnectionHandler` instance
    pub fn new() -> Self {
        log::debug!("creating new ConnectionHandler");
        Self {
            connections: HashMap::new(),
            current_id: 0,
        }
    }

    /// handles a new `WSConnection`
    ///
    /// # Arguments
    ///
    /// *con: `WSConnection` - the connection to handle
    ///
    /// # Returns
    ///
    /// * res: `Result<(), String>` - wheter it worked or not
    ///
    /// # Errors
    ///
    /// If there is no place left for a connection, an error is returned: `"no place left"`.
    pub fn new_connection(&mut self, con: WSConnection) -> Result<(), String> {
        let start_id = self.current_id;
        let mut id = start_id;
        while self.connections.contains_key(&id) {
            id = (id + 1) % u64::max_value();
            if id == start_id {
                return Err("no place left".to_string());
            }
        }
        log::debug!("new connection with id #{} to ConnectionHandler", id);
        self.connections
            .insert(id, Arc::new(Mutex::new(BucketClient::new(id, con))));

        Ok(())
    }

    /// receives a message
    ///
    /// Loops through all clients and returns the first message it gets,
    /// if there is no message, None is returned.
    ///
    /// # Returns
    ///
    /// * message: `Option<BucketMessage>` - a message if one was received
    ///
    /// # None
    ///
    /// returns none if no client received a message
    pub fn receive_message(&mut self) -> Option<BucketMessage> {
        let mut kill_id = None;
        for (id, con) in self.connections.iter() {
            let res = con.lock().unwrap().try_recv();
            match res {
                Ok(message) => {
                    if let Some(mes) = message {
                        log::debug!("ConnectionHandler received a message for id #{}", id);
                        return Some(BucketMessage::new(con.clone(), mes)); // QUES: better way than cloning every time???
                    }
                }
                Err(err) => {
                    log::warn!(
                        "an error occured and a ConnectionHandler client will be removed: {}",
                        err
                    );
                    kill_id = Some(*id);
                }
            }
            if kill_id.is_some() {
                break;
            }
        }

        if let Some(id) = kill_id {
            self.disconnect_client(id);
            return self.receive_message(); // WARN: Dangerous???
        }

        None
    }

    /// disconnects the client with the provided id
    ///
    /// # Arguments
    ///
    /// * id: `u64` - the id of the client to disconnect
    pub fn disconnect_client(&mut self, id: u64) {
        self.connections
            .remove(&id)
            .unwrap_or_else(|| {
                log::error!("the client with the id #{} does not exist", id);
                panic!("the client with the id #{} does not exist", id);
            })
            .lock()
            .unwrap()
            .close_connection();
    }

    /// broadcastas a message to all clients
    /// 
    /// # Arguments
    /// 
    /// * message: `Vec<u8>` - the message to be broadcasted
    /// 
    /// # Returns
    /// 
    /// * result: `Result<(), tungstenite::error::Error>` - whether sending worked
    pub fn broadcast(&self, message: Vec<u8>) -> Result<(), tungstenite::error::Error> {
        for (_, client) in &self.connections {
            client.lock().unwrap().send(message.clone())?;
        }

        Ok(())
    }
}

/// wraps a server around a `Bucket` and hosts it
///
/// # Variables
///
/// * bucket: Arc<Mutex<dyn Bucket>>,
/// * ws_server: WebSocketServer,
/// * connection_handler: Arc<Mutex<ConnectionHandler>>,
/// * tick_ms: u64,
/// * running: Arc<AtomicBool>,
pub struct BucketServer {
    // TODO: data struct for running, tick rate, etc...
    bucket: Arc<Mutex<dyn Bucket>>,
    ws_server: WebSocketServer,
    connection_handler: Arc<Mutex<ConnectionHandler>>,
    tick_ms: u64,
    running: Arc<AtomicBool>,
}

impl BucketServer {
    // IDEA: start at new, and have a bool to pause and run => also not everything in Arc<Mutex<>>
    /// creates a new `BucketServer`
    ///
    /// # Arguments
    ///
    /// * bucket: `Arc<Mutex<dyn Bucket>>` - the bucket to host
    /// * ip: `String` - the ip where the `BucketServer` is hosted
    /// * port: `String` - the port where the `BucketServer is hosted
    /// * tick_ms: `u64` - the duration of a tick in ms
    ///
    /// # Returns
    ///
    /// * instance: `BucketServer` - a new `BucketServer` instance
    pub fn new(bucket: Arc<Mutex<dyn Bucket>>, ip: String, port: String, tick_ms: u64) -> Self {
        // QUES: pass WebSocketServer or ip and port???, QUES: Arc<Mutex<>> or Box<> ?
        log::debug!("creating new BucketServer at {}:{}", &ip, &port);
        Self {
            bucket,
            ws_server: WebSocketServer::new(ip, port),
            connection_handler: Arc::new(Mutex::new(ConnectionHandler::new())),
            tick_ms,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// starts the `WebServer` instance
    ///
    /// # Returns
    ///
    /// handle: `Option<thread::JoinHandle<std::io::Result<()>>>` - if the server was already started it returns `None`
    /// else `Some(thread::JoinHandle<std::io::Result<()>>)`
    pub fn start(&mut self) -> Option<thread::JoinHandle<std::io::Result<()>>> {
        let running = self.running.clone();
        if running.load(Ordering::SeqCst) {
            log::warn!(
                "The BucketServer at {}:{} is already running",
                self.ws_server.get_ip(),
                self.ws_server.get_port()
            );
            return None;
        }
        running.store(true, Ordering::SeqCst);
        let connection_handler = self.connection_handler.clone();
        self.ws_server.start(move |con| {
            log::error!("new connection");
            connection_handler
                .lock()
                .unwrap()
                .new_connection(con)
                .unwrap_or_else(|err| {
                    log::warn!(
                        "An error occured, while a client was connecting to a BucketServer: {}",
                        err
                    );
                });
        });

        let bucket = self.bucket.clone();
        let tick_ms = self.tick_ms;
        let connection_handler = self.connection_handler.clone();
        Some(thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(tick_ms)); // TODO: change to a tick rate
                bucket.lock().unwrap().update();

                while running.load(Ordering::SeqCst) {
                    if let Some(message) = connection_handler.lock().unwrap().receive_message()
                    {
                        log::debug!("[REM] message received from a BucketServer");
                        bucket.lock().unwrap().handle_message(message);
                    } else {
                        break;
                    }
                }
            }

            Ok(())
        }))
    }
}
