use std::sync::{Arc, Mutex};
use std::str;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::logic::traits_bucket_server::{Bucket};
use crate::logic::bucket_server::{BaseBucketMessage, BaseConnectionHandler};

use crate::logic::bucket_manager::BaseBucketManagerData;

pub struct ApiBucket {
    connection_handler: Arc<Mutex<BaseConnectionHandler>>,
    bucket_manager: Arc<Mutex<BaseBucketManagerData>>,
}

impl ApiBucket { // IDEA: NEXT: add bucket data and state
    pub fn new(connection_handler: Arc<Mutex<BaseConnectionHandler>>, bucket_manager: Arc<Mutex<BaseBucketManagerData>>) -> Self {
        Self {
            connection_handler,
            bucket_manager,
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
        let _ = client.clone().lock().unwrap();
        let msg = message.get_content();
        let content = str::from_utf8(&msg).unwrap(); // PROB: error handling
        if let Ok(api_request) = serde_json::from_str::<APIRequest>(content) {
            match api_request {
                APIRequest::JoinDivGameNormal => {
                    log::info!("client joined a normal div game");
                    client.lock().unwrap().send(serde_json::to_vec(&APIResponse::JoinGame("some_id".to_string())).unwrap()); // PROB: error handling // QUES: efficiency?
                    let id = client.lock().unwrap().get_id(); // QUES: two times lock bad?
                    self.connection_handler.lock().unwrap().disconnect_client(id);
                    log::debug!("Client left ApiBucket");
                },
                APIRequest::GetOpenLobbies => {
                    log::debug!("client asked for open lobbies");
                    client.lock().unwrap().send(serde_json::to_vec(&APIResponse::OpenLobbies(vec!(r#"{"id": "some_id", "max_players": 8, "current_players": 4}"#.to_string()))).unwrap())
                },
                APIRequest::GetRunningGames => {
                    log::debug!("client asked for open lobbies");
                    client.lock().unwrap().send(serde_json::to_vec(&APIResponse::RunningGames(vec!(r#"{"id": "some_id", "players_start": 8, "current_players": 4, "ticks": 243}"#.to_string()))).unwrap())
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
    GetRunningGames,
    GetOpenLobbies,
}

#[derive(Serialize, Deserialize, Debug)]
enum APIResponse {
    InvalidJson,
    InvalidRequest,
    JoinGame(String),
    OpenLobbies(Vec<String>), // TODO: Vec<GameMetaData>
    RunningGames(Vec<String>), // TODO: Vec<GameMetaData>
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