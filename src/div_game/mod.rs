use std::sync::{Arc, Mutex};
use std::str;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::logic::Bucket;
use crate::logic::bucket_server::{BaseBucketMessage, BaseConnectionHandler, BaseBucketData};

use crate::logic::bucket_manager::BaseBucketManagerData;

pub struct DivGameBucket {
    connection_handler: Arc<Mutex<BaseConnectionHandler>>,
    bucket_manager: Arc<Mutex<BaseBucketManagerData>>,
}

impl DivGameBucket {
    pub fn new(connection_handler: Arc<Mutex<BaseConnectionHandler>>, bucket_manager: Arc<Mutex<BaseBucketManagerData>>) -> Self {
        Self {
            connection_handler,
            bucket_manager,
        }
    }
}

impl Bucket for DivGameBucket {
    fn start(&mut self) {
        log::info!("DivGameBucket started");
    }

    fn stop(&mut self) {
        log::info!("DivGameBucket stoped");
    }

    fn handle_message(&mut self, mut message: BaseBucketMessage) { //}, bucket_manager: Arc<Mutex<BaseBucketManager>>) {
        log::info!("DivGameBucket received a message: {}", str::from_utf8(&message.get_content()).unwrap());
        let client = message.get_client();
        let msg = message.get_content();

        let content = str::from_utf8(&msg).unwrap(); // PROB: error handling
        if let Ok(api_request) = serde_json::from_str::<DivGameRequest>(content) {
            match api_request {
                DivGameRequest::Lobby(lobby_request) => {
                    match lobby_request {
                        DivGameLobbyRequest::Ready => {
                            log::debug!("client is ready");
                        },
                        DivGameLobbyRequest::NotReady => {
                            log::debug!("client is not ready");
                        },
                        _ => {
                            log::warn!("invalid APIRequest send to APIServer");
                            client.lock().unwrap().send(serde_json::to_vec(&DivGameResponse::InvalidRequest).unwrap());
                        },
                    }
                },
                // APIRequest::JoinDivGameNormal => {
                //     log::info!("client joined a normal div game");
                //     client.lock().unwrap().send(serde_json::to_vec(&APIResponse::JoinGame("some_id".to_string())).unwrap()); // PROB: error handling // QUES: efficiency?
                //     let id = client.lock().unwrap().get_id(); // QUES: two times lock bad?
                //     self.connection_handler.lock().unwrap().disconnect_client(id);
                //     log::debug!("client left ApiBucket");
                // },
                // APIRequest::JoinDivGameDirect(id) => {
                //     log::info!("client joined a normal div game direct");
                //     client.lock().unwrap().send(serde_json::to_vec(&APIResponse::JoinGame(id)).unwrap()); // PROB: error handling // QUES: efficiency?
                //     let id = client.lock().unwrap().get_id(); // QUES: two times lock bad?
                //     self.connection_handler.lock().unwrap().disconnect_client(id);
                //     log::debug!("client left ApiBucket");
                // },
                // APIRequest::GetOpenLobbies => {
                //     let lobbies = self.bucket_manager.lock().unwrap().get_open_lobbies();
                //     client.lock().unwrap().send(serde_json::to_vec(&APIResponse::OpenLobbies(lobbies)).unwrap())
                // },
                // APIRequest::GetRunningGames => {
                //     log::debug!("client asked for open lobbies");
                //     let games = self.bucket_manager.lock().unwrap().get_running_games();
                //     client.lock().unwrap().send(serde_json::to_vec(&APIResponse::RunningGames(games)).unwrap())
                // },
                _ => {
                    log::warn!("invalid APIRequest send to APIServer");
                    client.lock().unwrap().send(serde_json::to_vec(&DivGameResponse::InvalidRequest).unwrap());
                },
            }
        } else { // Prob: QUES: WARN: differentiate between invalid json and invalid request
            log::warn!("An error occured when parsing message");
            client.lock().unwrap().send(serde_json::to_vec(&DivGameResponse::InvalidJson).unwrap()); // PROB: error handling // QUES: efficiency?
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum DivGameRunningRequest {
    Test,
}

#[derive(Serialize, Deserialize, Debug)]
enum DivGameLobbyRequest {
    Ready,
    NotReady,
    Leave,
}

#[derive(Serialize, Deserialize, Debug)]
enum DivGameRequest {
    Running(DivGameRunningRequest),
    Lobby(DivGameLobbyRequest),
}

#[derive(Serialize, Deserialize, Debug)]
enum DivGameRunningResponse {
    GameEnd, // TODO: send some score/ranking
}

#[derive(Serialize, Deserialize, Debug)]
enum DivGameLobbyResponse {
    StartGame,
    LobbyStatus,
}

#[derive(Serialize, Deserialize, Debug)]
enum DivGameResponse {
    InvalidJson,
    InvalidRequest,
    Running(DivGameRunningResponse),
    Lobby(DivGameLobbyResponse),
}