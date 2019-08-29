#![warn(clippy::all)]

// use std::collections::{HashMap, VecDeque};
use std::net::{TcpListener, TcpStream};
// use std::sync::mpsc;
// use std::sync::mpsc::{Receiver, Sender};
// use std::sync::{Arc, Mutex};
// use std::thread;

// use common::protocol::error_handling::{ServerError, ServerErrorType};
// use common::protocol::{
//     read_package, send_package, Client, DataType, Package, PackageType, Request, ResponseType,
// };

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("listening started, ready to accept");

    for stream in listener.incoming() {
        println!("new connection established");
        let stream = stream?;
        
        let message = b"21020311";
        // let _ = stream.write(&message)?;
        // stream.flush().unwrap();
    }

    Ok(())
}