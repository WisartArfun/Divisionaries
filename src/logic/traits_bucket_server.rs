use std::sync::{Arc, Mutex};

use crate::connection::ConnectionServer;
use crate::connection::HandleNewConnection;
use crate::connection::Connection;

pub trait BucketServer<H: HandleNewConnection, B: Bucket, S: ConnectionServer> { // QUES: generics here or in method
    fn new(ip: &str, port: &str) -> Self;

    fn start(&mut self); // WARN: return handle
}

pub trait BucketClient { // when struct when trait?
    fn new(id: i64, connection: impl Connection) -> Self where Self: Sized;

    fn close_connection(&mut self);
}


pub trait BucketMessage {
    fn new(sender: Arc<Mutex<impl BucketClient>>, content: Vec<u8>) -> Self;

    fn get_content() -> Vec<u8>;

    fn get_client() -> Arc<Mutex<dyn BucketClient>>;
}

pub trait Bucket { // QUES: WARN: functino or whole trait generic???
    fn new() -> Self where Self: Sized;

    fn start(&mut self);

    fn stop(&mut self);

    fn handle_message(&mut self, message: impl BucketMessage) where Self: Sized; // QUES: why sized needed??? // QUES: why is & needed even when with &
}