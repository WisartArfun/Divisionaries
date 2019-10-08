use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use ctrlc;

use bucketer::http_server::trait_run_http_server::RunHttpServer;
use bucketer::logger::SimpleLogger;
use bucketer::http_server;

use bucketer::logic::bucket_server::{BaseBucketServer, BaseConnectionHandler};
use bucketer::logic::bucket_manager::BaseBucketManager;

use bucketer::api::ApiBucket;

fn main() -> std::io::Result<()> {
    let mode = "RUN";

    match mode {
        "RUN" => {
            let running = Arc::new(AtomicBool::new(true));
            let r = running.clone();

            ctrlc::set_handler(move || {
                r.store(false, Ordering::SeqCst);
            }).expect("Error setting Ctrl-C handler");


            SimpleLogger::init("config/log4rs.yaml");
            log::info!("Main thread running");

            let mut bucket_manager = BaseBucketManager::new();

            let connection_handler = Arc::new(Mutex::new(BaseConnectionHandler::new()));
            let api_bucket = Arc::new(Mutex::new(ApiBucket::new(connection_handler.clone(), bucket_manager.get_data())));
            let mut api_bucket = BaseBucketServer::new("localhost", "8005", api_bucket, connection_handler); // IDEA: directly in here
            let handle_api = api_bucket.start(running.clone());

            let mut server = http_server::server::HttpGameServer::new("localhost", "8200"); // IDEA: load ip and port from config
            let handle_http = server.start();

            // WARN: add try_join in loop
            if let Err(e) = handle_http.join().unwrap() {
                log::error!("An error occured while joining the http_server:\n\t{:?}", e);
                panic!("");
            }
            if let Err(e) = handle_api.join().unwrap() {
                log::error!("An error occured while joining the api_bucket:\n\t{:?}", e);
                panic!("");
            }
        },
        "TEST" => {
            let vec: Vec<u8> = vec!(1, 2, 3);
            println!("{:?} at {:p}", &vec, &vec);
            let vec2 = vec;
            println!("{:?} at {:p}", &vec2, &vec2);

            let mut vec3 = vec2;
            vec3.push(4);
            println!("{:?} at {:p}", &vec3, &vec3);

        },
        _ => {
            panic!("{} is not a valid mode", mode)
        },
    }
    

    Ok(())
}