use std::collections::{HashMap, HashSet};
use std::str;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use serde_json;

use crate::logic::bucket_server::{BaseBucketData, BaseBucketMessage, BaseConnectionHandler};
use crate::logic::Bucket;

use crate::logic::bucket_manager::BaseBucketManagerData;


pub struct DivGameBucket {
    connection_handler: Arc<Mutex<BaseConnectionHandler>>,
    bucket_manager: Arc<Mutex<BaseBucketManagerData>>,
    bucket_data: BaseBucketData,
}

impl DivGameBucket {
    pub fn new(
        connection_handler: Arc<Mutex<BaseConnectionHandler>>,
        bucket_manager: Arc<Mutex<BaseBucketManagerData>>,
        bucket_data: BaseBucketData,
    ) -> Self {
        Self {
            connection_handler,
            bucket_manager,
            bucket_data,
        }
    }
}

impl Bucket for DivGameBucket {
    fn start(&mut self) {
        log::info!("Bucket started");
    }

    fn stop(&mut self) {
        log::info!("Bucket stoped");
    }

    fn update(&mut self) {
    }

    fn handle_message(&mut self, mut message: BaseBucketMessage) {
        let msg = message.get_content();
        log::info!(
            "DivGameBucket received a message: {}",
            str::from_utf8(&msg).unwrap()
        );
        let client = message.get_client();
        let client_id = client.lock().unwrap().get_id();

        // if let Ok(api_request) = serde_json::from_slice::<DivGameRequest>(&msg) {}
    }
}

/// holds the `Map` struct and all needed types
mod game_map {

    use tile::Tile;

    /// Represents the state of the game map and handles actions.
    /// 
    /// * Variables
    /// 
    /// * tiles: `Vec<Vec<Tile>>` - state of map
    pub struct Map {
        tiles: Vec<Vec<Tile>>,
    }

    impl map {
        pub fn new() -> Self {
            unimplemented!();
        }

        /// Takes a `action::Action` and executes it.
        /// 
        /// # Arguments
        /// 
        /// * action: `action::Action` - an action to be executed
        /// 
        /// # Returns
        /// 
        /// * worked: `bool` - wheter the action was executed succesfully or not
        pub fn take_action(action: action::Action) -> bool {
            unimplemented!();
        }
    }

    /// organizes the `Tile` type
    mod tile {
        /// Holds all possible tiles of a map.
        pub enum Tile {
            King(King),
            Ground(Ground),
            City(City),
            Swamp(Swamp),
        }

        impl Tile {
            /// handles other `Tile` walking on this `Tile`
            /// 
            /// # Arguments
            /// 
            /// * trrops: `i64` - amount of troops walking on this `Tile`
            /// * color: `Color` - color of originating `Tile`
            /// 
            /// # Returns
            /// 
            /// * worked: `bool` - returns wether walking was succesful or not
            fn walk_on(&mut self, troops: i64, color: Color) -> bool {
                unimplemented!();
            }
        }

        struct King;
        struct Ground;
        struct City;
        struct Swamp;
    }

    /// defines what actions a player can take
    pub mod action {
        pub enum Action {
            Walk({from: (i32, i32), to: (i32, i32), frac: f32}),
        }
    }

    /// handles settings for generating the map and configuring the game
    pub mod settings {
        pub struct Settings;
    }

    /// stores metadata about the state of the game
    pub mod info {
        pub struct Info;
    }
}

mod protocol {
    use game_map::action::Action;
    use game_map::settings::Settings;
    use game_map::info::Info;
    use game_map::tile::Tile;

    pub enum Request {
        Lobby(LobbyRequest),
        Running(RunningRequest),
    }

    pub enum LobbyRequest {
        Join,
        Ready,
        NotReady,
        Quit,
        Settings(Settings),
    }

    pub enum RunningRequest {
        take_action(Action),
        Quit,
    }

    pub enum Response {
        Joined(Settings),
        StartGame(Map),
        State({info: Info, tiles: Vec<Vec<Tile>>}),
        StateUpdate({info: Info, changed_tiles: Vec<((i32, i32), Tile)>),
    }
}