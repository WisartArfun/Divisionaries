use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;
use std::time;
use std::str;
use std::sync::atomic::{AtomicBool, Ordering};

use serde::{Serialize, Deserialize};

use tungstenite;

use crate::logic::Bucket;

use crate::connection::ConnectionServer;
use crate::connection::Connection;

use crate::websocket_server::ws_connection::WSConnection;
use crate::websocket_server::WebSocketServer;

pub struct BaseBucketServer {
    connection_handler: Arc<Mutex<BaseConnectionHandler>>,
    bucket: Arc<Mutex<dyn Bucket>>,
    bucket_data: BaseBucketData,
    ws_server: WebSocketServer,
    ip: String,
    port: String,
    running: bool,
}

impl BaseBucketServer {
    // QUES: why no lifetime problems with A<M<dyn Bucket>> but with dyn Bucket???
    pub fn new(ip: &str, port: &str, bucket: Arc<Mutex<dyn Bucket>>, bucket_data: BaseBucketData, connection_handler: Arc<Mutex<BaseConnectionHandler>>) -> Self {
        log::info!("new BucketServer, ip: {}, port: {}", ip, port);
        BaseBucketServer{
            connection_handler,
            bucket,
            bucket_data,
            ws_server: WebSocketServer::new(ip, port),
            ip: ip.to_string(),
            port: port.to_string(),
            running: false
        }
    }

    pub fn start(&mut self, running: Arc<AtomicBool>) -> thread::JoinHandle<std::io::Result<()>> { // PROB: better solution
        self.ws_server.start(self.connection_handler.clone()); // WARN: check if already started

        let connection_handler = self.connection_handler.clone();
        let bucket = self.bucket.clone();
        
        let port = self.port.clone();
        let id = self.bucket_data.get_id();
        log::info!("starting BaseBucketServer with id: {}", &id);
        let handle = thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                thread::sleep(time::Duration::from_millis(200));
                
                while running.load(Ordering::SeqCst) {
                    let message = connection_handler.lock().unwrap().receive_message(&id);
                    if let Some(mut res) = message {
                        log::debug!("BaseBucketServer received a message from port {}: {:?}", &port, &res.get_content());
                        bucket.lock().unwrap().handle_message(res); //, bucket_manager.clone());
                    } else {
                        break;
                    }
                }
            }

            Ok(())
        });

        handle
    }

    pub fn get_bucket_data(&self) -> BaseBucketData {
        // self.bucket.lock().unwrap().get_bucket_data()
        self.bucket_data.clone()
    }
}

pub struct BaseBucketMessage {
    sender: Arc<Mutex<BaseBucketClient>>,
    content: Vec<u8>,
}

impl BaseBucketMessage {
    pub fn new(sender: Arc<Mutex<BaseBucketClient>>, content: Vec<u8>) -> Self {
        log::info!("new BaseBucketMessage was created"); // QUES: better identifier
        BaseBucketMessage{
            sender,
            content,
        }
    }

    pub fn get_content(&mut self) -> Vec<u8> {
        self.content.clone()
    }

    pub fn get_client(&mut self) -> Arc<Mutex<BaseBucketClient>> {
        self.sender.clone()
    }
}


pub struct BaseBucketClient {
    id: i64,
    connection: WSConnection,
    ready: bool,
}

impl BaseBucketClient {
    pub fn new(id: i64, connection: WSConnection) -> Self {
        log::info!("new BucketClient was created");
        BaseBucketClient {
            id,
            connection,
            ready: false,
        }
    }

    pub fn get_id(&mut self) -> i64 {
        self.id
    }

    pub fn get_ready(&mut self) -> bool {
        self.ready
    }

    pub fn set_ready(&mut self, ready: bool) {
        self.ready = ready;
    }

    pub fn try_recv(&mut self) -> Result<Option<Vec<u8>>, tungstenite::error::Error> {
        self.connection.try_recv()
    }

    pub fn send(&mut self, content: Vec<u8>) {
        log::debug!("BaseBucketClient is sending a message");
        self.connection.send(content);
    }

    pub fn close_connection(&mut self) {
        log::debug!("BaseBucketClient is closing a connection");
        self.connection.close();
    }
}

pub struct BaseConnectionHandler {
    pub connections: HashMap<i64, Arc<Mutex<BaseBucketClient>>>, // PROB: BucketClient hanging around that are no more on the list // QUES: BucketClient cannot be made into an object if not sized
    available_ids: Vec<i64>,
    highest_id: i64,
}

impl BaseConnectionHandler {
    pub fn new() -> Self {
        log::info!("new BaseConnectionHandler was created");
        BaseConnectionHandler{
            connections: HashMap::new(),
            available_ids: Vec::new(),
            highest_id: 0,
        }
    }

    pub fn handle_new_connection(&mut self, connection : WSConnection) {
        log::debug!("handling a new connection");
        let id: i64;
        if let Some(unused_id) = self.available_ids.pop() {
            id = unused_id;
        } else {
            id = self.highest_id;
            self.highest_id += 1;
        }
        log::debug!("BaseConnectionHandler is handling a new connection with id: {}", &id);
        let client = Arc::new(Mutex::new(BaseBucketClient::new(id, connection)));
        self.connections.insert(id, client);
        log::debug!("amount of connections: {}", self.connections.len());
    }

    pub fn disconnect_client(&mut self, id: i64) {
        if let Some(client) = self.connections.remove(&id) {
            client.lock().unwrap().close_connection();
            self.available_ids.push(id);
        } else {
            panic!("Client does not exist"); // PROB: nice handling
        }
    }

    pub fn receive_message(&mut self, game_id: &str) -> Option<BaseBucketMessage> {
        let mut id_kill = None;
        for (id, org_client) in (&self.connections).iter() { // QUES: iter vs normal??? // PROB: keep track of order => not every time the same one
            let client = org_client.clone();
            let message_res = client.lock().unwrap().try_recv();

            let mut kill = false;
            match message_res {
                Ok(message) => {
                    if let Some(mes) = message {
                        log::info!("BaseConnectionHandler received a message from id: {}", &game_id);
                        return Some(BaseBucketMessage::new(client, mes));
                    }
                },
                Err(err) => {
                    log::warn!("an error occured and client is being removed: {}", err);
                    id_kill = Some(*id);
                    kill = true;
                }
            }
            if kill {break;}
        }

        if let Some(id) = id_kill {
            self.disconnect_client(id);
        }

        None
    }

    pub fn broadcast(&mut self, message: Vec<u8>) {
        for (_, org_client) in (&self.connections).iter() {
            let client = org_client.clone();
            client.lock().unwrap().send(message.clone());
        }
    }
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BaseBucketData { // get this passed to constructor
    id: String,
    ip: String,
    port: String,
    max_user_size: i64,
    current_users: i64,
}

impl BaseBucketData {
    pub fn new<S: Into<String>>(id: S, ip: S, port: S, max_user_size: i64) -> Self {
        Self {
            id: id.into(),
            ip: ip.into(),
            port: port.into(),
            max_user_size,
            current_users: 0,
        }
    }

    pub fn get_id(&mut self) -> String {
        self.id.clone()
    }

    pub fn get_ip(&mut self) -> String {
        self.ip.clone()
    }

    pub fn get_port(&mut self) -> String {
        self.port.clone()
    }

    pub fn get_current_users(&mut self) -> i64 {
        self.current_users.clone()
    }

    pub fn increment_current_users(&mut self) {
        self.current_users += 1;
    }

    pub fn decrement_current_users(&mut self) {
        self.current_users -= 1;
    }

    pub fn get_max_user_size(&mut self) -> i64 {
        self.max_user_size
    }
}