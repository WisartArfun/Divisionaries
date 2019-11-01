use std::str;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use serde_json;

use bucketer::logic::bucket_server::{BaseBucketData, BaseBucketMessage, BaseConnectionHandler};
use bucketer::logic::Bucket;

use bucketer::logic::bucket_manager::BaseBucketManagerData;

pub use state::settings::Settings;
use state::State;

/// A bucket for a Divisionaries game
pub struct DivGameBucket {
    connection_handler: Arc<Mutex<BaseConnectionHandler>>,
    bucket_manager: Arc<Mutex<BaseBucketManagerData>>,
    bucket_data: BaseBucketData,
    game_state: State,
    settings: Settings,
}

impl DivGameBucket {
    /// creates a new `DivGameBucket`
    pub fn new(
        connection_handler: Arc<Mutex<BaseConnectionHandler>>,
        bucket_manager: Arc<Mutex<BaseBucketManagerData>>,
        bucket_data: BaseBucketData,
        settings: Settings,
    ) -> Self {
        Self {
            connection_handler,
            bucket_manager,
            bucket_data,
            game_state: State::new(),
            settings,
        }
    }
}

impl Bucket for DivGameBucket {
    /// starts the bucket
    fn start(&mut self) {
        log::info!("starting DivGameBucket");
        self.bucket_data.set_running(true);
        self.game_state.set_running(true);
    }

    /// stops the bucket
    fn stop(&mut self) {
        log::info!("stopping DivGameBucket");
        self.game_state.set_running(false);
        self.bucket_data.set_running(false); // QUES: also deconstruct???
    }

    /// updates the state of the bucket, in this case it does a turn in the game
    fn update(&mut self) {
        if !self.game_state.get_running() { return; }
        

        self.game_state.increment_turn();
    }

    /// handles messages
    ///
    /// # Arguments
    ///
    /// * message: `BaseBucketMessage`: informatino about message and sender
    fn handle_message(&mut self, mut message: BaseBucketMessage) {
        let msg = message.get_content();
        log::info!(
            "DivGameBucket received a message: {}",
            str::from_utf8(&msg).expect("could not convert msg to utf8")
        );
        let client = message.get_client();
        let client_id = client.lock().unwrap().get_id();

        unimplemented!();

        // if let Ok(api_request) = serde_json::from_slice::<DivGameRequest>(&msg) {}
    }
}

/// holds all types needed to describe the game state
mod state {
    use tile::Tile;
    use info::Info;
    use player::Player;

    /// Manages the state of the map, info about the game and the players.
    pub struct State {
        map: GameMap,
        info: Info,
        players: Vec<Player>,
    }

    impl State {
        /// creates a new `State` instance
        pub fn new() -> Self {
            Self {
                map: GameMap::new(),
                info: Info::new(),
                players: Vec::new(),
            }
        }

        /// increments the turn counter of the game saved in `Info`
        pub fn increment_turn(&mut self) {
            self.info.increment_turn()
        }

        /// checks whether the game is running
        pub fn get_running(&self) -> bool {
            self.info.get_running()
        }

        /// sets wheter the game runs or not
        pub fn set_running(&mut self, running: bool) {
            self.info.set_running(running);
        }
    }

    /// Represents the state of the game map and handles actions.
    ///
    /// * Variables
    ///
    /// * tiles: `Vec<Vec<Tile>>` - state of map
    pub struct GameMap {
        tiles: Vec<Vec<Tile>>,
    }

    impl GameMap {
        /// creates a new GameMap
        pub fn new() -> Self {
            Self {
                tiles: Vec::new(),
            }
        }

        /// generates map depending on settings
        pub fn generate_map(&mut self) {
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
        pub fn take_action(&mut self, action: action::Action) -> bool {
            unimplemented!();
        }
    }

    /// organizes the `Tile` type
    pub mod tile {
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
            fn walk_on(&mut self, troops: i64, color: super::Color) -> bool {
                unimplemented!();
            }
        }
        /// `King` struct for `Tile`
        struct King;

        /// `Ground` struct for `Tile`
        struct Ground;

        /// `City` struct for `Tile`
        struct City;

        /// `Swamp` struct for `Tile`
        struct Swamp;
    }

    /// defines what actions a player can take
    pub mod action {
        /// holds all possible actions a player can make
        pub enum Action {
            /// defines a step, form which `Tile` to which `Tile` and what percentage of the troops will move
            ///
            /// # Parameters
            ///
            /// * (): `{from: (i32, i32), to: (i32, i32), frac: f32}` - anonymous struct that holds from to and frac
            ///     * from: `(i32, i32)` - x, y of origin `Tile` in `GameMap`
            ///     * to: `(i32, i32)` - x, y of target `Tile` in `GameMap`
            ///     * frac: `f32` - percentage of troops to move from origin to target
            Walk {
                from: (i32, i32),
                to: (i32, i32),
                frac: f32,
            },
        }
    }

    /// handles settings for generating the map and configuring the game
    pub mod settings {
        pub struct Settings;

        impl Settings {
            pub fn new() -> Self {
                Self {}
            }
        }
    }

    /// stores metadata about the state of the game
    pub mod info {
        /// Saves data about the state of the game.Settings
        /// 
        /// # Variables
        /// 
        /// * turn: `i64` - turn counter 
        pub struct Info {
            turn: i64,
            running: bool,
        }

        impl Info {
            /// creates a new instance of Info
            pub fn new() -> Self {
                Self {
                    turn: 0,
                    running: false,
                }
            }

            /// increments the turn counter by one
            pub fn increment_turn(&mut self) {
                self.turn += 1; // WARN: check overflow
            }

            /// checks whether the game is running
            pub fn get_running(&self) -> bool {
                self.running
            }

            /// sets wheter the game runs or not
            pub fn set_running(&mut self, running: bool) {
                self.running = running
            }
        }
    }

    /// handles a player
    mod player {
        /// stores info about player and allows communication
        pub struct Player;
    }

    /// defines possilbe colors
    pub enum Color {
        Red,
        Green,
        Blue,
        Yellow,
    }
}

/// defines valid types to be sent
mod protocol {
    use super::state::{action::Action, info::Info, settings::Settings, tile::Tile};

    /// types of requests
    pub enum Request {
        Lobby(LobbyRequest),
        Running(RunningRequest),
    }

    /// requests that can be made while in lobby
    pub enum LobbyRequest {
        Join,
        Ready,
        NotReady,
        Quit,
        Settings(Settings),
    }

    /// requests that can be made while game running
    pub enum RunningRequest {
        TakeAction(Action),
        Quit,
    }

    /// possbile responses from server
    pub enum Response {
        Joined(Settings),
        StartGame {
            info: Info,
            tiles: Vec<Vec<Tile>>,
        },
        State {
            info: Info,
            tiles: Vec<Vec<Tile>>,
        },
        StateUpdate {
            info: Info,
            changed_tiles: Vec<((i32, i32), Tile)>,
        },
    }
}
