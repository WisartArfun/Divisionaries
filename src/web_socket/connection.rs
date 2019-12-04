/// handles a connection to a `WebSocketServer`

use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use tungstenite;

/// stores and manipulates a WebSocket
/// 
/// # Variables
/// 
/// * ws_conn: `Arc<Mutex<tungstenite::protocol::WebSocket<TcpStream>>>` - the connection
pub struct WSConnection {
    ws_conn: Arc<Mutex<tungstenite::protocol::WebSocket<TcpStream>>>,
}

impl WSConnection {
    /// creates a new `WSConnection`
    /// 
    /// # Arguments
    /// 
    /// * ws_conn: `tungstenite::protocol::WebSocket<TcpStream>` - the connection
    /// 
    /// # Returns
    /// 
    /// * an instance of `WSConnection` that handles `ws_conn`
    pub fn new(ws_conn: tungstenite::protocol::WebSocket<TcpStream>) -> Self {
        log::info!("creating new WSConnection");
        Self {
            ws_conn: Arc::new(Mutex::new(ws_conn))
        }
    }
    
    /// closes the connection
    pub fn close(&mut self) { // WARN: do this with a trait // WARN: return error for better practice
        log::info!("closing WSConnection");
        self.ws_conn.lock().unwrap().close(None).unwrap_or_else(|err| {
            log::warn!("a problem occured while closing a WSConnection: {}", err);
        });
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
    pub fn send(&mut self, message: Vec<u8>) -> Result<(), tungstenite::error::Error> { // QUES: return other error type
        log::debug!("sending message over WSConnection");
        self.ws_conn.lock().unwrap().write_message(tungstenite::Message::Binary(message))?;
        Ok(())
    }

    /// tries to receive a message over the connection, but does not block if there is no message to receive
    /// 
    /// # Returns
    /// 
    /// * message: `Result<Option<Vec<u8>>, tungstenite::error::Error>` - an `Option` with the message if one was received, if an error occured while receiving a message, a `tungstenite::error::Error` is returned
    pub fn try_recv(&mut self) -> Result<Option<Vec<u8>>, tungstenite::error::Error> {
        match self.ws_conn.lock().unwrap().read_message() {
            Ok(msg) => {
                log::debug!("received message from WSConnection.websocket");
                if msg.is_binary() || msg.is_text() {
                    return Ok(Some(msg.into_data()));
                }
            },
            // QUEST: correct error handling
            Err(e) => {
                if let tungstenite::error::Error::Io(err) = &e {
                    if err.kind() == std::io::ErrorKind::WouldBlock {
                        return Ok(None);
                    };
                }
                log::warn!("an error occured while receiving a message from WSConnection: {}", &e);
                return Err(e);
            }
        };

        Ok(None)
    }
}
