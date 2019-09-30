use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::logic::traits_bucket_server::*;

use crate::connection::ConnectionServer;
use crate::connection::HandleNewConnection;
use crate::connection::Connection;

pub struct BaseBucketServer<H: HandleNewConnection, B: Bucket, S: ConnectionServer> {
    connections: Arc<Mutex<H>>,
    bucket: B,
    ws_server: S,
    ip: String,
    port: String,
    running: bool,
}

// QUES: ugly that BaseBucketServer needs type parameters???
impl<H: HandleNewConnection, B: Bucket, S: ConnectionServer> BucketServer<H, B, S> for BaseBucketServer<H, B, S> { // WARN: types are declared twice: here and in struct decleration
    fn new(ip: &str, port: &str) -> Self { // QUES: why type parameter static?
        log::info!("new websocket, ip: {}, port: {}", ip, port);
        BaseBucketServer{connections: Arc::new(Mutex::new(H::new())), bucket: B::new(), ws_server: S::new(ip, port), ip: ip.to_string(), port: port.to_string(), running: false}
    }

    fn start(&mut self) { // return handle
        unimplemented!();
    }
}

struct BaseBucket {

}

impl Bucket for BaseBucket {
    fn new() -> Self {
        unimplemented!();
    }

    fn start(&mut self) {
        unimplemented!();
    }

    fn stop(&mut self) {
        unimplemented!();
    }

    fn handle_message(&mut self, _message: impl BucketMessage) {
        unimplemented!();
    }
}

struct BaseBucketMessage {

}

impl BucketMessage for BaseBucketMessage {
    fn new(_sender: Arc<Mutex<impl BucketClient>>, _content: Vec<u8>) -> Self {
        unimplemented!();
    }

    fn get_content() -> Vec<u8> {
        unimplemented!();
    }

    fn get_client() -> Arc<Mutex<dyn BucketClient>> {
        unimplemented!();
    }
}


struct BaseBucketClient {

}

impl BucketClient for BaseBucketClient { // when struct, when trait???
    fn new(_id: i64, _connection: impl Connection) -> Self {
        unimplemented!();
    }

    fn close_connection(&mut self) {
        unimplemented!();
    }
}

struct BaseConnectionHandler {
    connections: HashMap<i64, Arc<Mutex<dyn BucketClient>>>, // PROB: BucketClient hanging around that are no more on the list // QUES: BucketClient cannot be made into an object if not sized
    available_ids: Vec<i64>,
    highest_id: i64,
}

impl HandleNewConnection for BaseConnectionHandler {  
    fn new() -> Self {
        unimplemented!();
    }

    fn handle_new_connection(&mut self, _connection: impl Connection) { // QUES: dyn Connection => trait cannot be made into an object
        unimplemented!();
    }
}