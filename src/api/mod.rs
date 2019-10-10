use std::sync::{Arc, Mutex};
use std::str;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::logic::Bucket;
use crate::logic::bucket_server::{BaseBucketMessage, BaseConnectionHandler, BaseBucketData, BaseBucketServer};

use crate::logic::bucket_manager::BaseBucketManagerData;

use crate::div_game::DivGameBucket;

pub struct ApiBucket {
    connection_handler: Arc<Mutex<BaseConnectionHandler>>,
    bucket_manager: Arc<Mutex<BaseBucketManagerData>>,
}

impl ApiBucket {
    pub fn new(connection_handler: Arc<Mutex<BaseConnectionHandler>>, bucket_manager: Arc<Mutex<BaseBucketManagerData>>) -> Self {
        Self {
            connection_handler,
            bucket_manager,
        }
    }

    fn creat_new_div_game_normal(&mut self, id: &str, port: &str, bucket_data: BaseBucketData) {
        log::info!("creating new div game normal");
        let gm = Arc::new(Mutex::new(DivGameBucket::new(self.connection_handler.clone(), self.bucket_manager.clone())));
        let server = BaseBucketServer::new(id, port, gm, bucket_data, self.connection_handler.clone());
        self.bucket_manager.lock().unwrap().open_lobby(id.to_string(), server);
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
                    let ip = "127.0.0.1".to_string();
                    let port = "8022".to_string();
                    self.creat_new_div_game_normal(&game_id, &port, BaseBucketData::new(&game_id, &ip, &port, 4));

                    client.lock().unwrap().send(serde_json::to_vec(&APIResponse::JoinGame(game_id)).unwrap()); // PROB: error handling // QUES: efficiency?
                    let id = client.lock().unwrap().get_id(); // QUES: two times lock bad?
                    self.connection_handler.lock().unwrap().disconnect_client(id);
                    log::debug!("client left ApiBucket");
                },
                APIRequest::JoinDivGameDirect(game_id) => {
                    log::info!("client joined a normal div game direct");

                    let ip = "127.0.0.1".to_string();
                    let port = "8022~".to_string();
                    self.creat_new_div_game_normal(&game_id, &port, BaseBucketData::new(&game_id, &ip, &port, 4));

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