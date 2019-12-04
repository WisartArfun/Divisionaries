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
pub struct ConnectionHandler { // QUES: private again
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
        if let Some(client) = self.connections.remove(&id) {
            client.lock().unwrap().close_connection();
        } else {
            log::error!("no client with id #{} does not exist", id);
            panic!("no client with id #{} does not exist", id); // PROB: nice handling
        }
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

/// configures a `BucketServer`
/// 
/// # Derives
/// 
/// * Clone
#[derive(Clone)]
pub struct BucketData {
    name: String,
    id: u64,
    ip: String,
    port: String,
    tick_ms: u64,
    max_players: u64,
}

impl BucketData {
    /// creates a new `BucketData` instance
    /// 
    /// # Arguments
    /// 
    /// * name: `String` - the name of the `BucketSever`
    /// * id: `u64` - the id of the `BucketServer`
    /// * ip: `String` - the ip of the `BucketServer`
    /// * port: `String` - the port of the `BucketServer`
    /// * tick_ms: `u64` - the duration between two ticks in ms
    /// * max_players: `u64` - the maximum amount of allowed players
    pub fn new(name: String, id: u64, ip: String, port: String, tick_ms: u64, max_players: u64) -> Self {
        log::debug!("creating new BucketData instance at {}:{} ...", &ip, &port);
        Self {
            name,
            id,
            ip,
            port,
            tick_ms,
            max_players,
        }
    }

    /// return the name of the `BucketData` instance
    /// 
    /// # Returns
    /// 
    /// * name: `&str` - the name of the `BucketData` instance
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// return the id of the `BucketData` instance
    /// 
    /// # Returns
    /// 
    /// * id: `u64` - the id of the `BucketData` instance
    pub fn get_id(&self) -> u64 {
        self.id
    }

    /// return the ip of the `BucketData` instance
    /// 
    /// # Returns
    /// 
    /// * ip: `&str` - the ip of the `BucketData` instance
    pub fn get_ip(&self) -> &str {
        &self.ip
    }

    /// return the port of the `BucketData` instance
    /// 
    /// # Returns
    /// 
    /// * name: `&str` - the name of the `BucketData` instance
    pub fn get_port(&self) -> &str {
        &self.port
    }

    /// return the tick rate in ms of the `BucketData` instance
    /// 
    /// # Returns
    /// 
    /// * tick_ms: `u64` - the tick rate in ms of the `BucketData` instance
    pub fn get_tick_ms(&self) -> u64 {
        self.tick_ms
    }

    /// return the max amount of players of the `BucketData` instance
    /// 
    /// # Returns
    /// 
    /// * max_players: `u64` - the max amount of players of the `BucketData` instance
    pub fn get_max_players(&self) -> u64 {
        self.max_players
    }
}

/// wraps a server around a `Bucket` and hosts it
pub struct BucketServer {
    // TODO: data struct for running, tick rate, etc...
    bucket: Arc<Mutex<dyn Bucket>>,
    bucket_data: BucketData,
    ws_server: WebSocketServer,
    connection_handler: Arc<Mutex<ConnectionHandler>>,
    running: Arc<AtomicBool>,
}

impl BucketServer {
    // IDEA: start at new, and have a bool to pause and run => also not everything in Arc<Mutex<>>
    /// creates a new `BucketServer`
    ///
    /// # Arguments
    ///
    /// * bucket: `Arc<Mutex<dyn Bucket>>` - the bucket to host
    /// * bucket_data: `BucketData` - data to configure the `BucketServer`
    /// * connection_handler: `Arc<Mutex<ConnectionHandler>>` - handles connections
    ///
    /// # Returns
    ///
    /// * instance: `BucketServer` - a new `BucketServer` instance
    // WARN: TODO: somehow not take the connection_handler from outside
    pub fn new(bucket: Arc<Mutex<dyn Bucket>>, bucket_data: BucketData, connection_handler: Arc<Mutex<ConnectionHandler>>) -> Self {
        // QUES: pass WebSocketServer or ip and port???, QUES: Arc<Mutex<>> or Box<> ?
        let ip = bucket_data.get_ip().to_string();
        let port = bucket_data.get_port().to_string();
        log::info!("creating new BucketServer at {}:{} called {}", &ip, &port, bucket_data.get_name());
        Self {
            bucket,
            bucket_data,
            ws_server: WebSocketServer::new(ip, port),
            // connection_handler: Arc::new(Mutex::new(ConnectionHandler::new())), WARN:
            connection_handler,
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
        let tick_ms = self.bucket_data.get_tick_ms();
        let connection_handler = self.connection_handler.clone();
        Some(thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(tick_ms)); // TODO: change to a tick rate
                bucket.lock().unwrap().update();

                while running.load(Ordering::SeqCst) {
                    let res = connection_handler.lock().unwrap().receive_message(); // WARN: needed like this, because con_han needed in handle message
                    if let Some(message) = res {
                        bucket.lock().unwrap().handle_message(message);
                    } else {
                        break;
                    }
                }
            }

            Ok(())
        }))
    }

    /// returns a reference to `bucket_data`
    /// 
    /// # Returns
    /// 
    /// * bucket_data: `&BucketData` - a reference to `bucket_data`
    pub fn get_bucket_data(&self) -> &BucketData {
        &self.bucket_data
    }
}
