
use std::collections::HashMap;

use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::logic::game;
use crate::http_server::ports::get_available_port;

pub struct GameManager {
    ip: String,
    lobbies: HashMap<String, Arc<Mutex<game::Game>>>,
    running_games: HashMap<String, Arc<Mutex<game::Game>>>,
}

lazy_static!{
    pub static ref GAMEMANAGER: Mutex<GameManager> = Mutex::new(GameManager{ip: "127.0.0.1".to_string(), lobbies: HashMap::new(), running_games: HashMap::new()});
}

impl GameManager {
    // fn new() -> Mutex<GameManager> { // new would be better than pub static
    //     // unsafe {
    //         return GAMEMANAGER; //???
    //     // }
    // }
    pub fn set_ip(&mut self, ip: String) { // CALL THIS!!!
        self.ip = ip;
    }

    pub fn get_game_lobby(&mut self, lobby_id: &str) -> Option<Arc<Mutex<game::Game>>> {
        if !self.lobbies.contains_key(lobby_id) {
            if let Some(available_port) = get_available_port(&self.ip) {
                let game_instance = game::Game::new(self.ip.clone(), available_port.to_string(), lobby_id.to_string());
                self.lobbies.insert(lobby_id.to_string(), Arc::new(Mutex::new(game_instance)));
            }
            else {
                return None;
            }
        }
        Some(self.lobbies.get(lobby_id).unwrap().clone())
    }

    pub fn get_running_game(&mut self, game_id: &str) -> Option<Arc<Mutex<game::Game>>> {
        if !self.running_games.contains_key(game_id) {
            return None;
        }
        Some(self.running_games.get(game_id).unwrap().clone())
    }

    pub fn start_lobby(&mut self, lobby_id: &str) { // result???
        if let Some(game_instance) = self.lobbies.remove(lobby_id) {
            self.running_games.insert(lobby_id.to_string(), game_instance);
        }

        // error
    }

    pub fn game_finished(&mut self, game_id: &str) {
        self.running_games.remove(game_id);
    }
}