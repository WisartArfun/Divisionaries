use std::{str, sync::{Arc, Mutex}, thread};

use serde::{Deserialize, Serialize};
use serde_json;

extern crate bucketer;
use bucketer::bucket::{Bucket, BucketData, ConnectionHandler, BucketManager, BucketServer, BucketMessage};

use crate::prog::tic_tac_toe::TicTacToe;

pub struct ApiBucket {
    connection_handler: Arc<Mutex<ConnectionHandler>>,
    bucket_manager: BucketManager,
    bucket_ip: String,
    num: u64,
}

impl ApiBucket {
    pub fn new(connection_handler: Arc<Mutex<ConnectionHandler>>, bucket_manager: BucketManager, bucket_ip: String) -> Self {
        Self {
            connection_handler,
            bucket_manager,
            bucket_ip,
            num: 1,
        }
    }

    // QUES: B may not live long enough when static??
    fn create_new_bucket_server(&mut self, bucket: Arc<Mutex<dyn Bucket>>, connection_handler: Arc<Mutex<ConnectionHandler>>, bucket_data: BucketData) -> Option<thread::JoinHandle<std::io::Result<()>>> {
        log::debug!("creating new BucketServer from Bucket");
        let mut bucket_server = BucketServer::new(bucket, bucket_data, connection_handler);
        let handle = bucket_server.start();
        self.bucket_manager.new_lobby(bucket_server);
        handle
    }

    fn create_tic_tac_toe(&mut self, id: String, ip: String, port: String) -> Option<thread::JoinHandle<std::io::Result<()>>> {
        log::info!("creating new TicTacToe...");
        let bucket_data = BucketData::new("TicTacToe".to_string(), self.num, ip, port, 40, 2);
        self.num += 1;
        let connection_handler = Arc::new(Mutex::new(ConnectionHandler::new()));
        let bucket = Arc::new(Mutex::new(TicTacToe::new(connection_handler.clone())));
        self.create_new_bucket_server(bucket, connection_handler, bucket_data)
    }

    fn check_game(&mut self, id: &str, name: &str) -> Option<thread::JoinHandle<std::io::Result<()>>> {
        if let Some(_) = self.bucket_manager.get_game_location(id) {
            return None;
        }
        
        let ip = self.bucket_ip.clone();
        let port = if let Some(port) = BucketManager::get_available_port(&ip) {
            port
        } else {
            log::error!("no ip available");
            panic!("no ip available");
        };

        match name {
            "TicTacToe" => self.create_tic_tac_toe(id.to_string(), ip, port),
            _ => {
                log::error!("unknown game type");
                panic!("unknown game type");
            }
        }
    }
}

impl Bucket for ApiBucket {
    fn start(&mut self) {
        log::info!("ApiBucket starting...");
    }

    fn stop(&mut self) {
        log::info!("ApiBucket stoping...");
    }

    fn handle_message(&mut self, message: BucketMessage) {
        log::info!("Api received a message: {}", str::from_utf8(&message.get_content()).expect("Invalid UTF-8"));
        let client = message.get_sender();
        let msg = message.get_content();
        
        let content = str::from_utf8(&msg).expect("invalid UTF-8"); // PROB: error handling
        let request = match serde_json::from_str::<Request>(content) {
            Ok(request) => request,
            Err(err) => {
                log::warn!("An error occured when parsing message: {}", err.to_string());
                client.lock().unwrap().send(serde_json::to_vec(&Response::InvalidRequest(err.to_string())).unwrap()).expect("sending message failed");
                return;
            },
        };
        match request {
            // what if name occupied by other game type???
            Request::JoinGame(name) => {
                log::debug!("request to API Bucket to join a game of type: {}", &name);
                self.check_game(&name, &name);

                if let Some(data) = self.bucket_manager.get_lobby_location(&name) {
                    let response = serde_json::to_vec(&Response::JoinGame(data.get_name().to_string())).expect("serialize failed");
                    client.lock().unwrap().send(response).expect("sending failed");
                    let id = client.lock().unwrap().get_id();
                    self.connection_handler.lock().unwrap().disconnect_client(id);
                } else {
                    let response = serde_json::to_vec(&Response::GameNotFound).expect("serialize failed");
                    client.lock().unwrap().send(response).expect("sending failed");
                }
            },
            _ => {
                log::error!("unimplemented api request");
                panic!("unimplemented api request");
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
enum Request {
    JoinGame(String), // name
    JoinGameDirect(String, String), // name, game_id
    GetLobbyLocation(String), // game_id
    GetGameLocation(String), // game_id
    GetRunningGames,
    GetOpenLobbies,
}

#[derive(Serialize)]
enum Response {
    InvalidRequest(String), // prob
    NotFound,
    LobbyLocation((String, String, String)), // game_id, ip, port
    GameLocation((String, String, String)), // game_id, ip, port
    JoinGame(String), // game_id
    GameNotFound,
    // OpenLobbies(Vec<BucketData>),
    // RunningGames(Vec<BucketData>),
}


// JAVASCRIPT TEST

// let socket = new WebSocket('ws://127.0.0.1:8001');
// let m = '"JoinDivGameNormal"';
// let tmp = undefined;
// let tmp2 = undefined;
// socket.onopen = function(event) {
// 	socket.send(m);

// 	socket.onmessage = function(event) {
// 		tmp = event;
// 		tmp.data.text().then(res => {
// 			tmp2 = res; console.log(res);
// 			console.log(event);
//         });
//     }

// 	socket.onclose = function(event) {
// 		console.log("connection closed");
// 		console.log(event);
//     }
// }