//! A collection of traits used for different kinds of connections.

use std::sync::{Arc, Mutex};

use crate::websocket_server::ws_connection::WSConnection;

use crate::logic::bucket_server::BaseConnectionHandler;

/// Trait to make new connections.
/// 
/// A `struct` that implements `Connection` can be treated connection.websocket_server.
/// This can be things such as websockets or files for testing.
/// 
/// # Examples
/// 
/// Rough implementation of `Connection` for a `struct` including a `tungstenite` websocket:
/// ```rust
/// use std::sync::{Arc, Mutex};
/// use std::net::TcpStream;
/// use tungstenite;
/// use std::io::{Read, Write};
/// 
/// use bucketer::connection::Connection;
/// 
/// struct WSConnection {
///    conn: tungstenite::protocol::WebSocket<TcpStream>,
/// }
///
/// impl Connection for WSConnection {
///     fn send(&mut self, message: Vec<u8>) {
///         self.conn.write_message(tungstenite::Message::Binary(message)).unwrap();
///     }
///
///     fn try_recv(&mut self) -> Option<Vec<u8>> {
///         match self.conn.read_message() {
///             Ok(msg) => return Some(msg.into_data()),
///             Err(ref _e) => {None},
///         }
///     }
/// }
/// ```
pub trait Connection { // QUES: Sized
    /// Takes a `Vec<u8>` and sends it over the connection.
    /// 
    /// # Arguments
    /// 
    /// * message: `Vec<u8>` - message to be sent
    fn send(&mut self, message: Vec<u8>) where Self: Sized; // QUES: better to use &[u8] ?

    /// Returns message as `Vec<u8>` if there is one, otherwise `None`. `try_recv` is therefore non-blocking.
    /// 
    /// # Returns
    /// 
    /// * message: `Option<Vec<u8>>` - `Some(message)` if there is one, else `None`
    fn try_recv(&mut self) -> Option<Vec<u8>> where Self: Sized;
}

pub trait UserServer { // QUES: stop???
    fn new(ip: &str, port: &str) -> Self;

    fn start_loop(&mut self);
}

/// The `HandleNewConnection` trait creates an object, which handles new connections.
pub trait HandleNewConnection { // QUES: with Box or lifetime (&'a mut dyn Connection)
    /// Initializes an object which will handle new connections.
    /// 
    /// # Returns
    /// 
    /// * _ : `HandleNewConnection` - an initialized object that implements `HandleNewConnection`
    fn new() -> Self where Self: Sized; // QUES: should new be in trait???

    /// Handles a new connection.
    /// 
    /// # Arguments
    /// 
    /// * connection: `impl Connection` - the connection that should be handled
    // fn handle_new_connection(&mut self, connection: impl Connection) where Self: Sized; // QUES: why this sized?
    fn handle_new_connection(&mut self, connection: WSConnection);

    fn disconnect_client(&mut self, id: i64);
}

// QUES: PROB: create a client trait???

/// Creates a `Message` object and handles sender and content.
/// 
/// A `Message` object knows about the sender and the content of the message.
/// It is responsible for providing all needed information to the receiver of the message.
/// 
/// # Type parameters
/// 
/// * C: _ - an object which represents a client
pub trait Message<C> { // QUES: correct solution, instead of creating a Client trait or something???
    /// Returns a new `Message` object.
    /// 
    /// # Arguments
    /// 
    /// * sender: `C` - a client object
    /// * content: `Vec<u8>` - the content which should be stored in the `Message` object
    /// 
    /// # Returns
    /// 
    /// _: `Message` - a new `Message` object
    fn new(sender: C, content: Vec<u8>) -> Self;
    
    // QUES: is `Message` object the right expression
    /// Returns the content of a `Message` object and takes ownership.
    /// 
    /// # Returns
    /// 
    /// * content: `Vec<u8>` - content of message
    fn get_content(&mut self) -> Vec<u8>; // QUES: take owenership of one variable???

    /// Return the sender of the message.
    /// 
    /// # Returns
    /// 
    /// * sender: `C` - the client which sent the `Message` object
    fn get_sender(&mut self) -> C;
}

/// Handles `Message` objects
/// 
/// Receives `Message` objects, handles their content
/// and if needed responds to the sender or changes the state in `Self`.
pub trait HandleMessage { // QUES: type parameters here or in method???
    /// Gets `Message` object and handles it
    /// 
    /// # Type parameters
    /// 
    /// * C: _ - a client object
    /// 
    /// # Arguments
    /// 
    /// * message: `impl Message<C>` - `Message` object to be handled
    fn handle_message<C>(&mut self, message: impl Message<C>);
}

pub trait ConnectionServer {
    fn new(ip: &str, port: &str) -> Self where Self: Sized;

    fn start(&mut self, callback: Arc<Mutex<BaseConnectionHandler>>);
    // fn start<T: HandleNewConnection + Send + 'static>(&mut self, callback: Arc<Mutex<T>>) where Self: Sized; // PROB: generics
    // fn start(&mut self, callback: Arc<Mutex<dyn HandleNewConnection>>) where Self: Sized; // PROB: the `handle_new_connection` method cannot be invoked on a trait object
}