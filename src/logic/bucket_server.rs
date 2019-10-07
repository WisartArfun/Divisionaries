use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;
use std::time;
use std::str;

use crate::logic::traits_bucket_server::*;

use crate::connection::ConnectionServer;
use crate::connection::HandleNewConnection;
use crate::connection::Connection;

use crate::websocket_server::ws_connection::WSConnection;
use crate::websocket_server::server::WebSocketServer;

use crate::logic::bucket_manager::BaseBucketManagerData;

// pub struct BaseBucketServer<H: HandleNewConnection + ReceiveMessage, B: Bucket<H>, S: ConnectionServer> {
// pub struct BaseBucketServer<B: Bucket> {
pub struct BaseBucketServer {
    // connection_handler: Arc<Mutex<H>>,
    connection_handler: Arc<Mutex<BaseConnectionHandler>>,
    // bucket: Arc<Mutex<B>>,
    bucket:Arc<Mutex<dyn Bucket>>,
    // ws_server: S,
    ws_server: WebSocketServer,
    ip: String,
    port: String,
    running: bool,
}

// QUES: ugly that BaseBucketServer needs type parameters??? // QUES: WARN: PROB: 'static lifetime
// impl<H: HandleNewConnection + ReceiveMessage + Send + 'static, B: Bucket<H> + Send + 'static, S: ConnectionServer> BucketServer<H, B, S> for BaseBucketServer<H, B, S> { // WARN: types are declared twice: here and in struct decleration
// impl<H: HandleNewConnection + ReceiveMessage + Send + 'static, B: Bucket<H> + Send + 'static, S: ConnectionServer> BaseBucketServer<H, B, S> { // WARN: types are declared twice: here and in struct decleration
// impl<B: Bucket + Send + 'static> BaseBucketServer<B> { // WARN: types are declared twice: here and in struct decleration
impl BaseBucketServer {
    // QUES: why no lifetime problems with A<M<dyn Bucket>> but with dyn Bucket???
    pub fn new(ip: &str, port: &str, bucket: Arc<Mutex<dyn Bucket>>) -> Self { //, bucket_manager: Arc<Mutex<BaseBucketManagerData>>) -> Self { // QUES: why type parameter static?
    // fn new(ip: &str, port: &str) -> Self { // QUES: why type parameter static?
        log::info!("new BucketServer, ip: {}, port: {}", ip, port);
        // let connection_handler = Arc::new(Mutex::new(H::new()));
        let connection_handler = Arc::new(Mutex::new(BaseConnectionHandler::new()));
        BaseBucketServer{
            connection_handler: connection_handler.clone(),
            // bucket: Arc::new(Mutex::new(B::new(connection_handler, bucket_manager))), //, bucket_manager))),
            // bucket: Arc::new(Mutex::new(bucket)),
            bucket,
            // bucket: Arc::new(Mutex::new(B::new(connection_handler, Arc::new(Mutex::new(BaseBucketManager::new()))))),
            // ws_server: S::new(ip, port),
            ws_server: WebSocketServer::new(ip, port),
            ip: ip.to_string(),
            port: port.to_string(),
            running: false
        }
    }

    // fn start<M: BucketMessage>(&mut self) { // return handle
    pub fn start(&mut self) -> thread::JoinHandle<std::io::Result<()>> { // PROB: better solution
        self.ws_server.start(self.connection_handler.clone()); // WARN: check if already started

        let connection_handler = self.connection_handler.clone();
        let bucket = self.bucket.clone();
        // let bucket_manager = bucket_manager.clone();

        let handle = thread::spawn(move || {
            loop {
                thread::sleep(time::Duration::from_millis(200));
                
                loop {
                    let message = connection_handler.lock().unwrap().receive_message();
                    if let Some(mut res) = message {
                        log::debug!("BaseBucketServer received a message: {:?}", &res.get_content());
                        bucket.lock().unwrap().handle_message(res); //, bucket_manager.clone());
                    } else {
                        break;
                    }
                }
            }
        });

        handle
    }
}

// pub struct BaseBucket<H: HandleNewConnection + ReceiveMessage> {
//     connection_handler: Arc<Mutex<H>>,
// }

// impl<H: HandleNewConnection + ReceiveMessage> Bucket<H> for BaseBucket<H> {
//     fn new(connection_handler: Arc<Mutex<H>>) -> Self { // better way
//         BaseBucket{connection_handler}
//     }

//     fn start(&mut self) {
//         log::info!("BaseBucket started");
//     }

//     fn stop(&mut self) {
//         log::info!("BaseBucket stoped");
//     }

//     // fn handle_message(&mut self, _message: impl BucketMessage) {
//     fn handle_message(&mut self, mut message: BaseBucketMessage) {
//         log::info!("BaseBucket received a message: {}", str::from_utf8(&message.get_content()).unwrap());
//     }
// }

pub struct BaseBucketMessage {
    sender: Arc<Mutex<BaseBucketClient>>,
    content: Vec<u8>,
}

// impl BucketMessage for BaseBucketMessage {
impl BaseBucketMessage {
    // fn new<C: Connection, B: BucketClient<C>>(sender: Arc<Mutex<B>>, content: Vec<u8>) -> Self {
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

    // fn get_client<C: Connection>() -> Arc<Mutex<dyn BucketClient<C>>> {
    // pub fn get_client<C: Connection>() -> Arc<Mutex<BaseBucketClient>> {
    pub fn get_client(&mut self) -> Arc<Mutex<BaseBucketClient>> {
        self.sender.clone()
    }
}


// struct BaseBucketClient<C: Connection> {
pub struct BaseBucketClient {
    id: i64,
    // connection: C, // WARN: make at least conn generic
    connection: WSConnection,
}

// impl<C: Connection> BucketClient<C> for BaseBucketClient<C> { // when struct, when trait???
// impl<C: Connection> BaseBucketClient<C> {
impl BaseBucketClient {
    // fn new(id: i64, connection: C) -> Self { // QUES: how not double but also not in trait
    pub fn new(id: i64, connection: WSConnection) -> Self {
        log::info!("new BucketClient was created");
        BaseBucketClient {
            id,
            connection,
        }
    }

    pub fn get_id(&mut self) -> i64 {
        self.id
    }

    pub fn try_recv(&mut self) -> Option<Vec<u8>> {
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
    // connections: HashMap<i64, Arc<Mutex<dyn BucketClient<C>>>>, // PROB: BucketClient hanging around that are no more on the list // QUES: BucketClient cannot be made into an object if not sized
    connections: HashMap<i64, Arc<Mutex<BaseBucketClient>>>,
    available_ids: Vec<i64>,
    highest_id: i64,
}

// impl HandleNewConnection for BaseConnectionHandler {  
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
    // fn handle_new_connection(&mut self, connection: impl Connection) { // QUES: dyn Connection => trait cannot be made into an object
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
    }

    pub fn disconnect_client(&mut self, id: i64) {
        println!("connections: {}", self.connections.len());
        if let Some(client) = self.connections.remove(&id) {
            client.lock().unwrap().close_connection();
            self.available_ids.push(id);
        } else {
            panic!("Client does not exist"); // PROB: nice handling
        }
    }
// }

// // impl ReceiveMessage for BaseConnectionHandler {
// impl BaseConnectionHandler {
    pub fn receive_message(&mut self) -> Option<BaseBucketMessage> {
        for (id, org_client) in (&self.connections).iter() { // iter vs normal??? // PROB: keep track of order => not every time the same one
            let client = org_client.clone();
            let message_res = client.lock().unwrap().try_recv();
            if let Some(message) = message_res {
                log::info!("BaseConnectionHandler received a message");
                return Some(BaseBucketMessage::new(client, message));
            }
        }

        None
    }
}