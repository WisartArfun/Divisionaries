use crate::logic::client;

use std::sync::{Arc, Mutex};

pub trait SecureAdd<T> {
    fn add(&mut self, object: T);
}

pub struct SecureList {
    pub clients: Vec<client::Client>,
}

pub struct Game {
    pub clients: Arc<Mutex<SecureList>>,
}

impl Game {
    pub fn new() -> Game {
        Game{clients: Arc::new(Mutex::new(SecureList{clients: Vec::new()}))}
    }
}

impl SecureAdd<client::Client> for SecureList {
    fn add(&mut self, client: client::Client) {
        println!("hello");
        self.clients.push(client);
    }
}