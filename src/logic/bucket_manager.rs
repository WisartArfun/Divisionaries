use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use log;

use crate::logic::traits_bucket_server::{Bucket}; //, ReceiveMessage, BucketServer};
use crate::logic::bucket_server::{BaseBucketServer};
use crate::connection::{HandleNewConnection, ConnectionServer, Connection};

// trait H: HandleNewConnection + ReceiveMessage {}
// pub struct BaseBucketManagerData {
//     lobbies: HashMap<String, Box<BucketServer<dyn H, dyn Bucket<dyn Connection>, dyn ConnectionServer>>>, // QUES: when does this work and when not? (sized and stuff) ???
//     games: HashMap<String, Box<BucketServer<dyn H, dyn Bucket<dyn Connection>, dyn ConnectionServer>>>, // 
// }
pub struct BaseBucketManagerData {
    lobbies: HashMap<String, BaseBucketServer>,
    games: HashMap<String, BaseBucketServer>,
}

impl BaseBucketManagerData {
    pub fn new() -> Self {
        log::info!("new BaseBucketManager created");
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

    // pub fn open_lobby(&mut self, id: String, lobby: Box<dyn BucketServer<dyn H, dyn Bucket<dyn Connection>, dyn ConnectionServer>>) {
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

    // pub fn open_lobby(&mut self, id: String, lobby: Box<dyn BucketServer<dyn H, dyn Bucket<dyn Connection>, dyn ConnectionServer>>) {
    pub fn open_lobby(&mut self, id: String, lobby: BaseBucketServer) {
        log::info!("opening a new lobby with id: {}", &id);
        self.data.lock().unwrap().open_lobby(id, lobby);
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
}