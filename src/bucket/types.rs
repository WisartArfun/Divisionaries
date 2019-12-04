//! base types for `Bucket`

use std::sync::{Arc, Mutex};

use tungstenite;

use crate::web_socket::WSConnection;

/// represents a message which a `Bucket` can handle
/// 
/// # Variables
/// 
/// * sender: `Arc<Mutex<BucketClient>>` - the sender of the message
/// * content: `Vec<u8>` - the content of the message
pub struct BucketMessage {
    sender: Arc<Mutex<BucketClient>>,
    content: Vec<u8>,
}

impl BucketMessage {
    /// creates a new instance of `BucketMessage`
    /// 
    /// # Arguments
    /// 
    /// * sender: `Arc<Mutex<BucketClient>>` - the sender of the message
    /// * content: `Vec<u8>` - the content of the message
    /// 
    /// # Returns 
    /// 
    /// * instance: `BucketMessage` - a new instance of `BucketMessage`
    pub fn new(sender: Arc<Mutex<BucketClient>>, content: Vec<u8>) -> Self {
        log::info!("creating BucketMessage");
        Self {
            sender,
            content,
        }
    }

    /// returns a clone of the content of the message
    /// 
    /// # Returns
    /// 
    /// * content: `Vec<u8>` - a clone of the content of the message
    pub fn get_content(&self) -> Vec<u8> {
        self.content.clone()
    }
    
    /// returns the sender of the message
    /// 
    /// # Returns
    /// 
    /// * sender: `Arc<Mutex<BaseBucketClient>>` - the sender of the message
    pub fn get_sender(&self) -> Arc<Mutex<BucketClient>> {
        self.sender.clone()
    }
}

/// represents a client of a `Bucket`
/// 
/// # Variables
/// 
/// * id: `u64` - the id of the client
/// * connection: `WSConnection` - the connection to the client
pub struct BucketClient {
    id: u64,
    connection: WSConnection,
}

impl BucketClient {
    /// creates a new instance of `BucketClient`
    /// 
    /// # Arguments
    /// 
    /// * id: `u64` - the id of the client
    /// * connection: `WSConnection` - the connection to the client
    pub fn new(id: u64, connection: WSConnection) -> Self {
        log::info!("creating new BucketClient");
        Self {
            id,
            connection,
        }
    }

    /// returns the id of the `BucketClient`
    /// 
    /// # Returns
    /// 
    /// * id: `u64` - the id of the client
    pub fn get_id(&self) -> u64 {
        self.id
    }

    /// tries to receive message from `self.connection`
    /// 
    /// # Returns
    /// 
    /// message: `Result<Option<Vec<u8>>, tungstenite::error::Error>` - an `Option` with the message if one was received, if an error occured while receiving a message, a `tungstenite::error::Error` is returned
    pub fn try_recv(&mut self) -> Result<Option<Vec<u8>>, tungstenite::error::Error> {
        self.connection.try_recv()
    }

    /// sends a message over the connection if possible, otherwise returns an `Error`
    /// 
    /// # Arguments
    /// 
    /// * message: `Vec<u8>` - the message to be sent over the connection
    /// 
    /// # Returns
    /// 
    /// * outcome: `Result<(), tungstenite::error::Error>` - wheter sending was successfull
    pub fn send(&mut self, message: Vec<u8>) -> Result<(), tungstenite::error::Error> {
        log::debug!("sending message over BucketClient");
        self.connection.send(message)
    }

    /// closes the connection
    pub fn close_connection(&mut self) {
        log::debug!("BucketClient #{} is closing connection to client", self.id);
        self.connection.close();
    }
}

/// characterizes a `Bucket`
/// 
/// # Derives
/// 
/// * Send
pub trait Bucket: Send {
    /// starts the `Bucket`
    fn start(&mut self);

    /// stops the `Bucket`
    fn stop(&mut self);

    /// updates the `Bucket`
    fn update(&mut self) {}

    /// handles a `BucketMessage`
    /// 
    /// # Arguments
    /// 
    /// * message: `BucketMessage` - the message to handle
    fn handle_message(&mut self, message: BucketMessage);
}
