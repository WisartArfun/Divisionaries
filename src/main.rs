use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;
use ctrlc;
use config;

use bucketer::logger::SimpleLogger;

use bucketer::http_server;
use bucketer::http_server::game_service_provider::GameServiceProvider;

use bucketer::logic::bucket_server::{BaseBucketServer, BaseConnectionHandler, BaseBucketData};
use bucketer::logic::bucket_manager::BaseBucketManager;

use bucketer::api::ApiBucket;

fn main() -> std::io::Result<()> {
    let mode = "RUN";

    match mode {
        "RUN" => {
            // initialize logger
            SimpleLogger::init("config/log4rs.yaml");
            log::info!("Main thread running");

            // ctrl-c handling => atomic bool that can stop all other threads
            log::info!("setting ctrl-c handler");
            let running = Arc::new(AtomicBool::new(true));
            let r = running.clone();

            ctrlc::set_handler(move || {
                r.store(false, Ordering::SeqCst);
            }).expect("Error setting Ctrl-C handler");
            
            // load config data
            log::info!("loading data from config file");
            let mut settings = config::Config::default();
            settings.merge(config::File::with_name("config/Settings")).unwrap(); // QUES: error handling
            let settings = settings.try_into::<HashMap<String, String>>().unwrap();

            let api_ip = if let Some(port) = settings.get("api_ip") {port} else {"localhost"};
            let api_port = if let Some(port) = settings.get("api_port") {port} else {"8001"};

            let http_ip = if let Some(port) = settings.get("http_ip") {port} else {"localhost"};
            let http_port = if let Some(port) = settings.get("http_port") {port} else {"8000"};
            
            //
            // IDEA: in BaseBucketServer save bucket and bucketdata seperately
            //

            // initialize bucket manager
            log::info!("creating bucket manager");
            let mut bucket_manager = BaseBucketManager::new();
            
            bucket_manager.create_api_bucket(api_ip, api_port, running.clone());
            // initializing bucket api
            // log::info!("creating api bucket");
            // let connection_handler = Arc::new(Mutex::new(BaseConnectionHandler::new()));
            // let api_bucket = Arc::new(Mutex::new(ApiBucket::new(connection_handler.clone(), bucket_manager.get_data(), BaseBucketData::new("API", 10_000))));
            // let mut api_bucket = BaseBucketServer::new(api_ip, api_port, api_bucket, connection_handler); // IDEA: directly in here
            // let _handle_api = api_bucket.start(running.clone());

            // bucket_manager.open_lobby("API".to_string(), api_bucket);

            // Initializing http server
            log::info!("creating http server");
            let mut server = http_server::HttpGameServer::new(http_ip, http_port); // IDEA: load ip and port from config
            let handle_http = server.start::<GameServiceProvider>();

            // handling started threads // WARN: add try_join in loop
            if let Err(e) = handle_http.join().unwrap() {
                log::error!("An error occured while joining the http_server:\n\t{:?}", e);
                panic!("");
            }
            // if let Err(e) = handle_api.join().unwrap() {
            //     log::error!("An error occured while joining the api_bucket:\n\t{:?}", e);
            //     panic!("");
            // }
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