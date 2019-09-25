mod http_server;
mod logger;
mod data_manager;
mod connection;
mod websocket_server;
mod api;
mod logic;

use log;

use http_server::trait_run_http_server::RunHttpServer;
use logger::simple_logger::SimpleLogger;
use logic::trait_game::Game;


fn main() -> std::io::Result<()> {
    let mode = "RUN";

    match mode {
        "RUN" => {
            SimpleLogger::init();
            log::info!("Main thread running");

            let mut server = http_server::server::HttpGameServer::new("localhost", "8000"); // load ip and port from config
            let handle = server.start();

            let mut api_server = api::server::APIServer::new("localhost", "8001");
            api_server.start();

            let mut nor_div_game_server = logic::normal_div_game::NormalDivGame::new("localhost", "8002");
            nor_div_game_server.start_server();

            if let Err(e) = handle.join().unwrap() {
                log::error!("An error occured while joining the http_server:\n\t{:?}", e);
                panic!("");
            }
        },
        "FM" => {
            // test file manager
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