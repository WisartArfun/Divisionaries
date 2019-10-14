use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::str;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::logic::Bucket;
use crate::logic::bucket_server::{BaseBucketMessage, BaseConnectionHandler, BaseBucketData, BaseBucketServer};

use crate::logic::bucket_manager::{BaseBucketManager, BaseBucketManagerData};

use crate::div_game::DivGameBucket;

pub struct ApiBucket {
    connection_handler: Arc<Mutex<BaseConnectionHandler>>,
    bucket_manager: Arc<Mutex<BaseBucketManagerData>>,
    running: Arc<AtomicBool>,
}

impl ApiBucket {
    pub fn new(connection_handler: Arc<Mutex<BaseConnectionHandler>>, bucket_manager: Arc<Mutex<BaseBucketManagerData>>, running: Arc<AtomicBool>) -> Self {
        Self {
            connection_handler,
            bucket_manager,
            running,
        }
    }

    // fn creat_new_div_game_normal(&mut self, id: &str, port: &str, bucket_data: BaseBucketData) {
    fn create_new_div_game_normal(&mut self, mut bucket_data: BaseBucketData) {
        log::info!("creating new div game normal");
        let bch = Arc::new(Mutex::new(BaseConnectionHandler::new()));
        let gm = Arc::new(Mutex::new(DivGameBucket::new(bch.clone(), self.bucket_manager.clone(), bucket_data.clone())));
        let id = bucket_data.get_id();
        let mut server = BaseBucketServer::new(&bucket_data.get_ip(), &bucket_data.get_port(), gm, bucket_data, bch);
        let _ = server.start(self.running.clone());
        self.bucket_manager.lock().unwrap().open_lobby(id, server);
    }

    fn check_div_game(&mut self, game_id: &str) {
        let (ip, port) = match self.bucket_manager.lock().unwrap().get_lobby_ip_port(&game_id) {
            Some(data) => data,
            None => {
                log::error!("No available port found");
                return; // QUES: PROB: send message to client
            },
        };
        
        if !self.bucket_manager.lock().unwrap().lobby_exists(game_id) {
            self.create_new_div_game_normal(BaseBucketData::new(game_id, &ip, &port, 4));
        }
    }
}

impl Bucket for ApiBucket {
    fn start(&mut self) {
        log::info!("ApiBucket started");
    }

    fn stop(&mut self) {
        log::info!("ApiBucket stoped");
    }

    fn handle_message(&mut self, mut message: BaseBucketMessage) { //}, bucket_manager: Arc<Mutex<BaseBucketManager>>) {
        log::info!("Api received a message: {}", str::from_utf8(&message.get_content()).unwrap());
        let client = message.get_client();
        let msg = message.get_content();
        
        let content = str::from_utf8(&msg).unwrap(); // PROB: error handling
        if let Ok(api_request) = serde_json::from_str::<APIRequest>(content) {
            match api_request {
                APIRequest::JoinDivGameNormal => {
                    log::info!("client joined a normal div game");

                    let game_id = "match".to_string();
                    self.check_div_game(&game_id);

                    client.lock().unwrap().send(serde_json::to_vec(&APIResponse::JoinGame(game_id)).unwrap()); // PROB: error handling // QUES: efficiency?
                    let id = client.lock().unwrap().get_id(); // QUES: two times lock bad?
                    self.connection_handler.lock().unwrap().disconnect_client(id);
                    log::debug!("client left ApiBucket");
                },
                APIRequest::JoinDivGameDirect(game_id) => {
                    log::info!("client joined a normal div game direct");

                    self.check_div_game(&game_id);

                    client.lock().unwrap().send(serde_json::to_vec(&APIResponse::JoinGame(game_id)).unwrap()); // PROB: error handling // QUES: efficiency?
                    let id = client.lock().unwrap().get_id(); // QUES: two times lock bad?
                    self.connection_handler.lock().unwrap().disconnect_client(id);
                    log::debug!("client left ApiBucket");
                },
                APIRequest::GetOpenLobbies => {
                    let lobbies = self.bucket_manager.lock().unwrap().get_open_lobbies();
                    client.lock().unwrap().send(serde_json::to_vec(&APIResponse::OpenLobbies(lobbies)).unwrap())
                },
                APIRequest::GetRunningGames => {
                    log::debug!("client asked for open lobbies");
                    let games = self.bucket_manager.lock().unwrap().get_running_games();
                    client.lock().unwrap().send(serde_json::to_vec(&APIResponse::RunningGames(games)).unwrap())
                },
                APIRequest::GetLobbyLocation(game_id) => {
                    log::debug!("client asked for lobby location");
                    let game_data = self.bucket_manager.lock().unwrap().get_lobby_location(&game_id);
                    if let Some(mut data) = game_data {
                        client.lock().unwrap().send(serde_json::to_vec(&APIResponse::LobbyLocation((data.get_id(), data.get_ip(), data.get_port()))).unwrap());
                    } else {
                        client.lock().unwrap().send(serde_json::to_vec(&APIResponse::NotFound).unwrap())
                    }
                },
                APIRequest::GetGameLocation(game_id) => {
                    log::debug!("client asked for game location");
                    let game_data = self.bucket_manager.lock().unwrap().get_game_location(&game_id);
                    if let Some(mut data) = game_data {
                        client.lock().unwrap().send(serde_json::to_vec(&APIResponse::GameLocation((data.get_id(), data.get_ip(), data.get_port()))).unwrap());
                    } else {
                        client.lock().unwrap().send(serde_json::to_vec(&APIResponse::NotFound).unwrap())
                    }
                },
                _ => {
                    log::warn!("invalid APIRequest send to APIServer");
                    client.lock().unwrap().send(serde_json::to_vec(&APIResponse::InvalidRequest).unwrap());
                },
            }
        } else { // Prob: QUES: WARN: differentiate between invalid json and invalid request
            log::warn!("An error occured when parsing message");
            client.lock().unwrap().send(serde_json::to_vec(&APIResponse::InvalidJson).unwrap()); // PROB: error handling // QUES: efficiency?
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum APIRequest {
    JoinDivGameNormal,
    JoinDivGameDirect(String), // game_id
    GetLobbyLocation(String), // game_id
    GetGameLocation(String), // game_id
    GetRunningGames,
    GetOpenLobbies,
}

#[derive(Serialize, Deserialize, Debug)]
enum APIResponse {
    InvalidJson,
    InvalidRequest,
    NotFound,
    LobbyLocation((String, String, String)), // game_id, ip, port
    GameLocation((String, String, String)), // game_id, ip, port
    JoinGame(String), // game_id
    OpenLobbies(Vec<BaseBucketData>),
    RunningGames(Vec<BaseBucketData>),
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