
use std::collections::HashMap;

use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::logic::game;
use crate::http_server::ports::get_available_port;

pub struct GameManager {
    running_games: HashMap<String, Arc<Mutex<game::Game>>>,
}

lazy_static!{
    pub static ref GAMEMANAGER: Mutex<GameManager> = Mutex::new(GameManager{running_games: HashMap::new()});
}

impl GameManager {
    // fn new() -> Mutex<GameManager> { // new would be better than pub static
    //     // unsafe {
    //         return GAMEMANAGER; //???
    //     // }
    // }
    pub fn get_game_instance(&mut self, game_id: &str) -> Option<Arc<Mutex<game::Game>>> {
        let ip = "localhost".to_string();
        if !self.running_games.contains_key(game_id) {
            if let Some(available_port) = get_available_port(&ip) {
                let game_instance = game::Game::new(ip, available_port.to_string());
                self.running_games.insert(game_id.to_string(), Arc::new(Mutex::new(game_instance)));
            }
            else {
                return None;
            }
        }
        Some(self.running_games.get(game_id).unwrap().clone())
    }
}