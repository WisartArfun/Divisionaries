//! manages `BucketServer` instances
use std::collections::HashMap;
use std::net::TcpListener;

use super::{BucketData, BucketServer}; // QUES: bad practice to import reexport?

/// manages `BucketServer` instances
// TODO: do it with a id
pub struct BucketManager {
    lobbies: HashMap<String, BucketServer>,
    games: HashMap<String, BucketServer>,
}

impl BucketManager {
    /// creates a new `BucketManager` instance
    /// 
    /// # Returns
    /// 
    /// * instance: `BucketManager` - new `BucketManager` instance
    pub fn new() -> Self {
        Self {
            lobbies: HashMap::new(),
            games: HashMap::new(),
        }
    }

    /// checks if a lobby with a given id exists
    /// 
    /// # Arguments
    /// 
    /// * id: `String` - the `id` of the lobby to check
    /// 
    /// # Returns
    /// 
    /// * exists: `bool` - `true` if a lobby with `id` exists, else `false`
    fn lobby_exists(&mut self, id: String) -> bool {
        if self.lobbies.contains_key(&id) {
            return true;
        }
        false
    }

    /// checks if a game with a given id exists
    /// 
    /// # Arguments
    /// 
    /// * id: `String` - the `id` of the game to check
    /// 
    /// # Returns
    /// 
    /// * exists: `bool` - `true` if a game with `id` exists, else `false`
    fn game_exists(&mut self, id: String) -> bool {
        if self.games.contains_key(&id) {
            return true;
        }
        false
    }

    /// creates a new lobby
    /// 
    /// # Arguments
    /// 
    /// * bucket_server: `BucketServer` - the bucket_server used to create a new lobby
    pub fn new_lobby(&mut self, bucket_server: BucketServer) {
        let id = bucket_server.get_bucket_data().get_name().to_string();
        log::info!(
            "BucketManager is creating a new lobby with the id: {} ...",
            &id
        );
        self.lobbies.insert(id, bucket_server);
    }

    /// turns a lobby into a game
    /// 
    /// # Arguments
    /// 
    /// * id: `String` - the id of the lobby to turn into a game
    /// 
    /// # Returns
    /// 
    /// * works: `Result<(), String>` - whether it worked or not
    /// 
    /// # Errors
    /// 
    /// returns an error if no lobby with the given id exists
    pub fn new_game(&mut self, id: String) -> Result<(), String> {
        log::debug!("starting a new game from lobby with id: {}", id);
        let bucket_server = match self.lobbies.remove(&id) {
            None => return Err("No matching id in lobbies".to_string()),
            Some(bucket_server) => bucket_server,
        };
        self.games.insert(id, bucket_server);
        Ok(())
    }

    // WARN: this does not stop the game
    /// closes a game
    /// 
    /// # Arguments
    /// 
    /// id: `&str` - the id of the game to close
    /// 
    /// # Returns
    /// 
    /// works: `Result<(), String>` - whether it worked or not
    /// 
    /// # Errors
    /// 
    /// returns an error if no game with the given id exists
    pub fn close_game(&mut self, id: &str) -> Result<(), String> {
        log::debug!("closing game with id: {}", id);
        if self.games.remove(id).is_none() {
            return Err("No matching id in games".to_string());
        }
        Ok(())
    }

    /// returns a `Vec` with the data about all lobbies
    /// 
    /// # Returns
    /// 
    /// data: `Vec<BucketData>` - data about all lobbies
    pub fn get_lobbies(&self) -> Vec<BucketData> {
        let mut bucket_servers: Vec<BucketData> = Vec::new();
        for (_, bucket_server) in &self.lobbies {
            bucket_servers.push(bucket_server.get_bucket_data().clone());
        }
        bucket_servers
    }

    /// returns a `Vec` with the data about all games
    /// 
    /// # Returns
    /// 
    /// data: `Vec<BucketData>` - data about all games
    pub fn get_games(&self) -> Vec<BucketData> {
        let mut bucket_servers: Vec<BucketData> = Vec::new();
        for (_, bucket_server) in &self.games {
            bucket_servers.push(bucket_server.get_bucket_data().clone());
        }
        bucket_servers
    }

    /// returns the data about a lobby with a certain id
    /// 
    /// # Arguments
    /// 
    /// * id: `&str` - the id of the lobby to look up
    /// 
    /// # Returns
    /// 
    /// data: `Option<BucketData>` - the data of the lobby
    /// 
    /// # None
    /// 
    /// returns none if no lobby with the given id exists
    pub fn get_lobby_location(&self, id: &str) -> Option<BucketData> {
        Some(self.lobbies.get(id)?.get_bucket_data().clone())
    }

    /// returns the data about a game with a certain id
    /// 
    /// # Arguments
    /// 
    /// * id: `&str` - the id of the game to look up
    /// 
    /// # Returns
    /// 
    /// data: `Option<BucketData>` - the data of the game
    /// 
    /// # None
    /// 
    /// returns none if no game with the given id exists
    pub fn get_game_location(&self, id: &str) -> Option<BucketData> {
        Some(self.games.get(id)?.get_bucket_data().clone())
    }

    /// checks if a certain address is available
    /// 
    /// # Arguments
    /// 
    /// * ip: `&str` - the ip to check
    /// * port: `&str` - the port to check
    /// 
    /// # Returns
    /// 
    /// works: `bool` - whether it worked or not
    fn address_is_available(ip: &str, port: &str) -> bool {
        // WARN: IDEA: TODO: seperate port management
        match TcpListener::bind(format!("{}:{}", ip, port)) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// returns an available port for a certain ip
    /// 
    /// # Arguments
    /// 
    /// ip: `&str` - the ip to get a port for
    /// 
    /// # Returns
    /// 
    /// port: `Option<String>` - an available port
    /// 
    /// # None
    /// 
    /// returns none if no port is available
    pub fn get_available_port(ip: &str) -> Option<String> {
        let port = (8002..9000).find(|port| Self::address_is_available(ip, &format!("{}", port)));
        if let Some(port) = port {
            return Some(format!("{}", port));
        }
        None
    }
}