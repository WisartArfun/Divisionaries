use std::sync::{Arc, Mutex};

use std::thread;

use crate::connection::ConnectionServer;
use crate::connection::HandleNewConnection;
// use crate::connection::Connection;

use crate::logic::bucket_server::BaseBucketMessage;

pub trait BucketServer<H: HandleNewConnection + ReceiveMessage, B: Bucket<H>, S: ConnectionServer> { // QUES: generics here or in method
    fn new(ip: &str, port: &str) -> Self;

    // fn start<M: BucketMessage>(&mut self); // WARN: return handle
    fn start(&mut self) -> thread::JoinHandle<std::io::Result<()>>;
}

// pub trait BucketClient<C: Connection> { // when struct when trait?
//     fn new(id: i64, connection: C) -> Self where Self: Sized;

//     fn try_recv(&mut self) -> Option<Vec<u8>>;

//     fn close_connection(&mut self);
// }


// pub trait BucketMessage {
//     fn new<C: Connection, B: BucketClient<C>>(sender: Arc<Mutex<B>>, content: Vec<u8>) -> Self;

//     fn get_content() -> Vec<u8>;

//     fn get_client<C: Connection>() -> Arc<Mutex<dyn BucketClient<C>>>;
// }

pub trait Bucket<H: HandleNewConnection + ReceiveMessage> { // QUES: WARN: functino or whole trait generic???
    fn new(connection_handler: Arc<Mutex<H>>) -> Self where Self: Sized;

    fn start(&mut self);

    fn stop(&mut self);

    fn handle_message(&mut self, message: BaseBucketMessage);
    // fn handle_message(&mut self, message: impl BucketMessage) where Self: Sized; // QUES: why sized needed??? // QUES: why is & needed even when with &
}

pub trait ReceiveMessage {
    fn receive_message(&mut self) -> Option<BaseBucketMessage>;
    // fn receive_message<M: BucketMessage>(&mut self) -> Option<Box<M>>;
}