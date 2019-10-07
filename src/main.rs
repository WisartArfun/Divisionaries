// mod http_server;
// mod logger;
mod data_manager;
// mod connection;
// mod websocket_server;
// mod api;
// extern crate bucketer;

// use std::fs::File;
// use std::io::Read;
// extern crate ruml;

// extern crate syn;

use std::sync::{Arc, Mutex};

use bucketer::http_server::trait_run_http_server::RunHttpServer;
use bucketer::logger::SimpleLogger;
use bucketer::http_server;

use bucketer::logic::bucket_server::{BaseBucketServer, BaseConnectionHandler};
use bucketer::logic::traits_bucket_server::{BucketServer};
use bucketer::logic::bucket_manager::BaseBucketManager;

use bucketer::websocket_server::server::WebSocketServer;

use bucketer::api::ApiBucket;

fn main() -> std::io::Result<()> {
    let mode = "RUN";

    match mode {
        "RUN" => {
            SimpleLogger::init("config/log4rs.yaml");
            log::info!("Main thread running");

            let mut bucket_manager = BaseBucketManager::new();

            let mut api_bucket = BaseBucketServer::<BaseConnectionHandler, ApiBucket<BaseConnectionHandler>, WebSocketServer>::new("localhost", "8001", bucket_manager.get_data());
            let handle_api = api_bucket.start();

            let mut server = http_server::server::HttpGameServer::new("localhost", "8000"); // load ip and port from config
            let handle_http = server.start();

            // WARN: add try_join in loop
            if let Err(e) = handle_api.join().unwrap() {
                log::error!("An error occured while joining the api_bucket:\n\t{:?}", e);
                panic!("");
            }
            if let Err(e) = handle_http.join().unwrap() {
                log::error!("An error occured while joining the http_server:\n\t{:?}", e);
                panic!("");
            }
        },
        // "UML" => {
        //     let mut file = File::open("src/main.rs").expect("Unable to open file");
        //     let mut src = String::new();
        //     file.read_to_string(&mut src).expect("Unable to read file");
        //     let file = syn::parse_file(&src).expect("Unable to parse file");
        //     let file = ruml::file_parser(file);
        //     println!("{}", ruml::render_plantuml(file));
        // },
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