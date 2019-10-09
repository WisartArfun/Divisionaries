use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use log;

use crate::logic::Bucket;
use crate::logic::bucket_server::{BaseBucketServer, BaseBucketData, BaseConnectionHandler};

use crate::api::ApiBucket;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct BaseBucketManagerData {
    lobbies: HashMap<String, BaseBucketServer>,
    games: HashMap<String, BaseBucketServer>,
}

impl BaseBucketManagerData {
    pub fn new() -> Self {
        log::info!("new BaseBucketManagerData created");
        Self {
            lobbies: HashMap::new(),
            games: HashMap::new(),
        }
    }

    pub fn lobby_exists(&mut self, id: &str) -> bool {
        if self.lobbies.contains_key(id) {return true}
        false
    }

    pub fn game_exists(&mut self, id: &str) -> bool {
        if self.games.contains_key(id) {return true}
        false
    }

    pub fn open_lobby(&mut self, id: String, lobby: BaseBucketServer) {
        log::info!("opening a new lobby with id: {}", &id);
        self.lobbies.insert(id, lobby);
    }

    pub fn start_lobby(&mut self, id: String) {
        log::info!("starting lobby with id: {}", &id);
        if !self.lobby_exists(&id) {panic!("add correct error handling")}

        let lobby = self.lobbies.remove(&id).unwrap();
        self.games.insert(id, lobby);
    }

    pub fn close_game(&mut self, id: String) {
        log::debug!("closing game with id: {}", &id);
        if !self.game_exists(&id) {panic!("add correct error handling")}

        let _ = self.games.remove(&id); // QUES: PROB: deconstructing correctly
    }

    pub fn get_running_games(&mut self) -> Vec<BaseBucketData> {
        let mut game_data = Vec::new();
        for (_, game) in &mut self.games {
            game_data.push(game.get_bucket_data());
        }
        game_data
    }

    pub fn get_open_lobbies(&mut self) -> Vec<BaseBucketData> {
        let mut lobby_data = Vec::new();
        for (_, lobby) in &mut self.lobbies {
            lobby_data.push(lobby.get_bucket_data());
        }
        lobby_data
    }
}

pub struct BaseBucketManager {
    data: Arc<Mutex<BaseBucketManagerData>>,
}

impl BaseBucketManager {
    pub fn new() -> Self {
        log::info!("new BaseBucketManager created");
        BaseBucketManager {
            data: Arc::new(Mutex::new(BaseBucketManagerData::new())),
        }
    }

    pub fn lobby_exists(&mut self, id: &str) -> bool {
        self.data.lock().unwrap().lobby_exists(id)
    }

    pub fn game_exists(&mut self, id: &str) -> bool {
        self.data.lock().unwrap().game_exists(id)
    }

    pub fn open_lobby(&mut self, id: String, lobby: BaseBucketServer) {
        log::info!("opening a new lobby with id: {}", &id);
        self.data.lock().unwrap().open_lobby(id, lobby);
        log::error!("heeerre");
    }

    pub fn start_lobby(&mut self, id: String) {
        log::info!("starting lobby with id: {}", &id);
        self.data.lock().unwrap().start_lobby(id);
    }

    pub fn close_game(&mut self, id: String) {
        log::debug!("closing game with id: {}", &id);
        self.data.lock().unwrap().close_game(id);
    }

    pub fn get_data(&mut self) -> Arc<Mutex<BaseBucketManagerData>> {
        log::debug!("getting BaseBucketManagerData clone from BaseBucketManager");
        self.data.clone()
    }

    pub fn create_api_bucket(&mut self, api_ip: &str, api_port: &str, running: Arc<AtomicBool>) {
        log::info!("creating api bucket");
        let connection_handler = Arc::new(Mutex::new(BaseConnectionHandler::new()));
        let api_bucket = Arc::new(Mutex::new(ApiBucket::new(connection_handler.clone(), self.get_data(), BaseBucketData::new("API", 10_000))));
        let mut api_bucket = BaseBucketServer::new(api_ip, api_port, api_bucket, connection_handler); // IDEA: directly in here
        let _handle_api = api_bucket.start(running);

        self.open_lobby("API".to_string(), api_bucket);
    }
}