use std::sync::{Arc, Mutex};
use std::str;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::logic::Bucket;
use crate::logic::bucket_server::{BaseBucketMessage, BaseConnectionHandler, BaseBucketData};

use crate::logic::bucket_manager::BaseBucketManagerData;

struct DivGameBucketState {
    pub clients: u64,
    pub ready: u64,
    pub running: bool,
}

impl DivGameBucketState {
    pub fn new() -> Self {
        Self {
            clients: 0,
            ready: 0,
            running: false,
        }
    }
}

pub struct DivGameBucket {
    connection_handler: Arc<Mutex<BaseConnectionHandler>>,
    bucket_manager: Arc<Mutex<BaseBucketManagerData>>,
    state: DivGameBucketState,
    bucket_data: BaseBucketData,
    game_running: bool,
}

impl DivGameBucket {
    pub fn new(connection_handler: Arc<Mutex<BaseConnectionHandler>>, bucket_manager: Arc<Mutex<BaseBucketManagerData>>, bucket_data: BaseBucketData) -> Self {
        Self {
            connection_handler,
            bucket_manager,
            state: DivGameBucketState::new(),
            bucket_data,
            game_running: false,
        }
    }
}

impl Bucket for DivGameBucket {
    fn start(&mut self) {
        log::info!("DivGameBucket started");
        self.game_running = true;
    }

    fn stop(&mut self) {
        log::info!("DivGameBucket stoped");
        self.game_running = false;
    }

    fn update(&mut self) {
        if !self.game_running {return;}
        
    }

    fn handle_message(&mut self, mut message: BaseBucketMessage) { //}, bucket_manager: Arc<Mutex<BaseBucketManager>>) {
        log::info!("DivGameBucket received a message: {}", str::from_utf8(&message.get_content()).unwrap());
        let client = message.get_client();
        let msg = message.get_content();

        let content = str::from_utf8(&msg).unwrap(); // PROB: error handling
        println!("{:?}", serde_json::to_string(&DivGameRequest::Lobby(DivGameLobbyRequest::Ready)));
        if let Ok(api_request) = serde_json::from_str::<DivGameRequest>(content) {
            match api_request {
                DivGameRequest::Lobby(lobby_request) => {
                    if self.state.running {
                        log::warn!("DivGameBucket received a lobby request although game is running");
                        return;
                    }
                    match lobby_request {
                        DivGameLobbyRequest::Ready => {
                            let ready = client.lock().unwrap().get_ready();
                            if ready {return;}

                            log::debug!("client is ready");
                            client.lock().unwrap().set_ready(true);
                            self.state.clients = self.connection_handler.lock().unwrap().connections.len() as u64;
                            self.state.ready += 1; // WARN: not safe
                            // if self.state.clients > 1 && self.state.ready * 3 > self.state.clients * 2 {
                            if self.state.ready * 3 > self.state.clients * 2 {
                                log::info!("starting game");
                                self.state.running = true;
                                let id = self.bucket_data.get_id();
                                self.bucket_manager.lock().unwrap().start_lobby(id);
                                self.connection_handler.lock().unwrap().broadcast(serde_json::to_vec(&DivGameResponse::Lobby(DivGameLobbyResponse::StartGame)).unwrap());
                            }
                        },
                        DivGameLobbyRequest::NotReady => {
                            log::debug!("client is not ready");
                            self.state.clients = self.connection_handler.lock().unwrap().connections.len() as u64;
                            self.state.ready -= 1;
                        },
                        _ => {
                            log::warn!("invalid APIRequest send to APIServer");
                            client.lock().unwrap().send(serde_json::to_vec(&DivGameResponse::InvalidRequest).unwrap());
                        },
                    }
                },
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