// std
use std::error::Error;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};

// extern
use log;
use ctrlc;

// own library
extern crate bucketer;
use bucketer::{logger, web_server::WebServer};
use bucketer::bucket::{BucketServer, BucketData, ConnectionHandler};

// bin
mod div;
use div::web_server::ServiceProvider;
use div::Config;
use div::bucket::TestBucket;

// bin tic_tac_toe
mod tic_tac_toe;
use tic_tac_toe::TicTacToe;

fn main() -> Result<(), Box<dyn Error>> {
    // initializing logger
    logger::init("config/log4rs.yaml");

    // setting ctrlc handler and creating atomicbool running variable for all threads
    log::info!("setting ctrl-c handler");
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        log::warn!("ctrl-c was pressed, the program will be terminated"); // why not running?
        r.store(false, Ordering::SeqCst); // QUES: local and then call stop on parts
    }).unwrap_or_else(|err| {
        log::error!("an error occured while setting ctrlc handler: {:?}", err);
        panic!("error while setting ctrlc handler: {:?}", err);
    });

    // loading config file
    let config = Config::new("config/Settings.toml").unwrap_or_else(|err| {
        log::error!("an error occured while reading the config file: {:?}", err);
        panic!("error while reading config file:\n\t: {:?}", err); // QUES: use default config?
    });

    // starting web server
    let mut web_server = WebServer::new(&config.http_ip, &config.http_port);
    let web_server_handle = web_server.start::<ServiceProvider>().unwrap(); // this is safe as it is the first time the web_server is started // QUES: change to result ???

    // // testing web socket
    // log::error!("starting web socket");
    // let mut web_socket = WebSocketServer::new(config.api_ip, config.api_port); // QUES: moves part of struct???
    // use std::sync::Mutex;
    // let test = Arc::new(Mutex::new(Vec::new()));
    // let closure_test = test.clone();
    // web_socket.start(move |inst| -> () {
    //     log::error!("closure running");
    //     closure_test.lock().unwrap().push(inst);
    // });

    // std::thread::sleep(std::time::Duration::from_secs(5));
    // log::error!("test size: {}", test.lock().unwrap().len());
    
    // testing bucket server
    log::info!("starting bucket server");
    let bucket_data = BucketData::new("TestBucket".to_string(), 1234567890, config.api_ip, config.api_port, 25, 5);
    let bucket_connection_handler = Arc::new(Mutex::new(ConnectionHandler::new()));
    let mut bucket_server = BucketServer::new(Arc::new(Mutex::new(TestBucket::new())), bucket_data, bucket_connection_handler);
    let bucket_server_handle = bucket_server.start().unwrap(); // this is safe as it is the first time bucket_server is started

    // testing tic-tac-toe bucket server
    log::info!("starting tic-tac-toe...");
    let tic_data = BucketData::new("TicTacToe".to_string(), 123, "127.0.0.1".to_string(), "8001".to_string(), 50, 2);
    let tic_connection_handler = Arc::new(Mutex::new(ConnectionHandler::new()));
    let tic_bucket = Arc::new(Mutex::new(TicTacToe::new(tic_connection_handler.clone())));
    let mut tic_server = BucketServer::new(tic_bucket, tic_data, tic_connection_handler);
    let tic_server_handle = tic_server.start().unwrap();

    // letting handles join
    if let Err(e) = web_server_handle.join() {
        log::error!("An error occured while joining the http_server:\n\t{:?}", e);
        panic!("Terminating program due to a fatal error:\n\t{:?}", e);
    }
    if let Err(e) = bucket_server_handle.join() {
        log::error!("An error occured while joining the bucket_server:\n\t{:?}", e);
        panic!("Terminating program due to a fatal error:\n\t{:?}", e);
    }

    // successfully stoping program
    Ok(())
}
