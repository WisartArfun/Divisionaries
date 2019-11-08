// std
use std::error::Error;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

// extern
use log;
use ctrlc;

// own library
use bucketer::{logger, web_server::WebServer};

// bin
mod div;
use div::web_server::ServiceProvider;
use div::Config;

fn main() -> Result<(), Box<dyn Error>> {
    // initializing logger
    logger::init("config/log4rs.yaml");

    // setting ctrlc handler and creating atomicbool running variable for all threads
    log::info!("setting ctrl-c handler");
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        log::warn!("ctrl-c was pressed, the program will be terminated"); // why not running?
        r.store(false, Ordering::SeqCst);
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

    // letting handles join
    if let Err(e) = web_server_handle.join() {
        log::error!("An error occured while joining the http_server:\n\t{:?}", e);
        panic!("Terminating program due to a fatal error:\n\t{:?}", e);
    }

    // successfully stoping program
    Ok(())
}
