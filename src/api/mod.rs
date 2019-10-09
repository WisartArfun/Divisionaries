use std::sync::{Arc, Mutex};
use std::str;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::logic::Bucket;
use crate::logic::bucket_server::{BaseBucketMessage, BaseConnectionHandler, BaseBucketData};

use crate::logic::bucket_manager::BaseBucketManagerData;

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

    // fn creat_new_div_game_normal(&mut self) {

    // }
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
                    client.lock().unwrap().send(serde_json::to_vec(&APIResponse::JoinGame("some_id".to_string())).unwrap()); // PROB: error handling // QUES: efficiency?
                    let id = client.lock().unwrap().get_id(); // QUES: two times lock bad?
                    self.connection_handler.lock().unwrap().disconnect_client(id);
                    log::debug!("client left ApiBucket");
                },
                APIRequest::JoinDivGameDirect(id) => {
                    log::info!("client joined a normal div game direct");
                    client.lock().unwrap().send(serde_json::to_vec(&APIResponse::JoinGame(id)).unwrap()); // PROB: error handling // QUES: efficiency?
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
    JoinDivGameDirect(String),
    GetRunningGames,
    GetOpenLobbies,
}

#[derive(Serialize, Deserialize, Debug)]
enum APIResponse {
    InvalidJson,
    InvalidRequest,
    JoinGame(String),
    OpenLobbies(Vec<BaseBucketData>),
    RunningGames(Vec<BaseBucketData>),
}


// JAVASCRIPT TEST

// let socket = new WebSocket('ws://localhost:8001');
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