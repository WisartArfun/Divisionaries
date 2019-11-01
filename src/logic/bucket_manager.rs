use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::net::TcpListener;

use log;

use crate::logic::Bucket;
use crate::logic::bucket_server::{BaseBucketServer, BaseBucketData, BaseConnectionHandler};

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
        log::debug!("getting open lobbies from BaseBucketManagerData");
        let mut lobby_data = Vec::new();
        for (_, lobby) in &mut self.lobbies {
            lobby_data.push(lobby.get_bucket_data());
        }
        lobby_data
    }

    pub fn get_lobby_location(&mut self, id: &str) -> Option<BaseBucketData> {
        log::debug!("getting lobby location from BaseBucketManagerData");
        if !self.lobby_exists(id) {
            return None;
        }

        Some(self.lobbies.get(id).unwrap().get_bucket_data())
    }

    pub fn get_game_location(&mut self, id: &str) -> Option<BaseBucketData> {
        log::debug!("getting game location from BaseBucketManagerData");
        if !self.game_exists(id) {
            return None;
        }
        
        Some(self.games.get(id).unwrap().get_bucket_data())
    }

    fn port_is_available(ip: &str, port: &str) -> bool { // QUES: WARN: do it in here or extern???
        match TcpListener::bind(format!("{}:{}", ip, port)) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn get_available_port(ip: &str) -> Option<String> {
        let port = (8000..9000)
            .find(|port| Self::port_is_available(ip, &format!("{}", port)));
        if let Some(port) = port {
            return Some(format!("{}", port));
        }
        None
    }

    pub fn get_lobby_ip_port(&mut self, id: &str) -> Option<(String, String)> {
        match self.lobbies.get(id) {
            Some(bucket) => Some((bucket.get_bucket_data().get_ip(), bucket.get_bucket_data().get_port())),
            None => {
                let ip = "127.0.0.1".to_string();
                match Self::get_available_port(&ip) {
                    Some(port) => Some((ip, port)),
                    None => None,
                }
            }
        }
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

    pub fn get_lobby_location(&mut self, id: &str) -> Option<BaseBucketData> {
        self.data.lock().unwrap().get_lobby_location(id)
    }

    pub fn get_game_location(&mut self, id: &str) -> Option<BaseBucketData> {
        self.data.lock().unwrap().get_game_location(id)
    }

    fn port_is_available(ip: &str, port: &str) -> bool { // QUES: WARN: do it in here or extern???
        BaseBucketManagerData::port_is_available(ip, port)
    }

    pub fn get_available_port(ip: &str) -> Option<String> {
        BaseBucketManagerData::get_available_port(ip)
    }

    pub fn get_lobby_ip_port(&mut self, id: &str) -> Option<(String, String)> {
        self.data.lock().unwrap().get_lobby_ip_port(id)
    }
}