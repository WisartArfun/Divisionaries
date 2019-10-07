use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use log;

use crate::logic::traits_bucket_server::{Bucket, ReceiveMessage, BucketServer};
use crate::connection::{HandleNewConnection, ConnectionServer, Connection};

trait H: HandleNewConnection + ReceiveMessage {}
pub struct BaseBucketManager {
    lobbies: HashMap<String, Arc<Mutex<&'static mut dyn BucketServer<dyn H, dyn Bucket<dyn Connection>, dyn ConnectionServer>>>>, // QUES: when does this work and when not? (sized and stuff) ???
    games: HashMap<String, Arc<Mutex<&'static mut dyn BucketServer<dyn H, dyn Bucket<dyn Connection>, dyn ConnectionServer>>>>, // 
}

unsafe impl Send for BaseBucketManager {}

// trait H: HandleNewConnection + ReceiveMessage {}
// trait BS: BucketServer<dyn H, dyn Bucket<dyn Connection>, dyn ConnectionServer> + Sized {}
impl BaseBucketManager {
    pub fn new() -> Self {
        log::info!("new BaseBucketManager created");
        BaseBucketManager {
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

    pub fn open_lobby(&mut self, id: String, lobby: Arc<Mutex<&'static mut dyn BucketServer<dyn H, dyn Bucket<dyn Connection>, dyn ConnectionServer>>>) {
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