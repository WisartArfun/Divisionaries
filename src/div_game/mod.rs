use std::sync::{Arc, Mutex};
use std::str;
use std::collections::{HashMap, hash_map::Drain};

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
    bucket_state: DivGameBucketState,
    state: Option<State>,
    bucket_data: BaseBucketData,
    game_running: bool,
}

impl DivGameBucket {
    pub fn new(connection_handler: Arc<Mutex<BaseConnectionHandler>>, bucket_manager: Arc<Mutex<BaseBucketManagerData>>, bucket_data: BaseBucketData) -> Self {
        Self {
            connection_handler,
            bucket_manager,
            bucket_state: DivGameBucketState::new(),
            state: None,
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
                    if self.bucket_state.running {
                        log::warn!("DivGameBucket received a lobby request although game is running");
                        return;
                    }
                    match lobby_request {
                        DivGameLobbyRequest::Ready => {
                            let ready = client.lock().unwrap().get_ready();
                            if ready {return;}

                            log::debug!("client is ready");
                            client.lock().unwrap().set_ready(true);
                            self.bucket_state.clients = self.connection_handler.lock().unwrap().connections.len() as u64;
                            self.bucket_state.ready += 1; // WARN: not safe
                            // if self.state.clients > 1 && self.state.ready * 3 > self.state.clients * 2 {
                            if self.bucket_state.ready * 3 > self.bucket_state.clients * 2 {
                                log::info!("starting game");
                                self.bucket_state.running = true;
                                let id = self.bucket_data.get_id();
                                self.bucket_manager.lock().unwrap().start_lobby(id);
                                let mut state = State::new(10, 10);
                                self.connection_handler.lock().unwrap().broadcast(serde_json::to_vec(&DivGameResponse::Lobby(DivGameLobbyResponse::StartGame(state.get_tiles()))).unwrap()); // QEUS: WARN: a lot of useless clone
                                state.make_move(Move::Step((2, 0), (2, 1), 1.0));
                                log::error!("move made");
                                let data = serde_json::to_vec(&DivGameResponse::Running(DivGameRunningResponse::StateUpdate(state.get_changes())));
                                match &data {
                                    Err(e) => println!("{:?}", e),
                                    _ => {},
                                }
                                let data = data.unwrap();
                                log::error!("JSON");
                                self.connection_handler.lock().unwrap().broadcast(data); // QEUS: WARN: a lot of useless clone
                                log::error!("sending");
                                // let state = State::new(10, 10);
                                // self.connection_handler.lock().unwrap().broadcast(r#"{"Running":{"StateUpdate":"111111"}}"#.to_string().into_bytes());
                                // self.connection_handler.lock().unwrap().broadcast(state.get_serialized_tiles().unwrap());
                                // self.connection_handler.lock().unwrap().broadcast(serde_json::to_vec(&DivGameResponse::Running(DivGameRunningResponse::State(state.get_tiles()))).unwrap()); // QUES: a lot of useless clone
                                self.state = Some(state);
                            }
                        },
                        DivGameLobbyRequest::NotReady => {
                            log::debug!("client is not ready");
                            self.bucket_state.clients = self.connection_handler.lock().unwrap().connections.len() as u64;
                            self.bucket_state.ready -= 1;
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
    State(Vec<Vec<Tile>>),
    // StateUpdate(Vec<(usize, usize, Tile)>),
    StateUpdate(HashMap<(usize, usize), Tile>),
}

#[derive(Serialize, Deserialize, Debug)]
enum DivGameLobbyResponse {
    StartGame(Vec<Vec<Tile>>),
    LobbyStatus,
}

#[derive(Serialize, Deserialize, Debug)]
enum DivGameResponse {
    InvalidJson,
    InvalidRequest,
    Running(DivGameRunningResponse),
    Lobby(DivGameLobbyResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Color {
    Empty,
    Blue,
    Green,
    Violet,
    Red,
    Cyan,
    DarkBlue,
    LightBlue,
    DarkGreen,
    LightGreen,
    Orange,
}

#[derive(Serialize, Deserialize, Debug)]
struct Score {
    players_eliminated: i64,
    fields_occupied: i64,
    soldiers: i64,
    soldiers_eliminated: i64,
    soldiers_lost: i64,
    cities_occupied: i64,
    troops_per_round: f64,
}

impl Score {
    pub fn new() -> Self {
        Self {
            players_eliminated: 0,
            fields_occupied: 0,
            soldiers: 0,
            soldiers_eliminated: 0,
            soldiers_lost: 0,
            cities_occupied: 0,
            troops_per_round: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Move {
    Step((usize, usize), (usize, usize), f32), // from, to, ratio
}

#[derive(Serialize, Deserialize, Debug)]
struct Player {
    color: Color,
    score: Score,
    moves: HashMap<i64, Vec<Move>>,
    // viewable: HashMap<(i64, i64), i16>,
}

impl Player {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            score: Score::new(),
            moves: HashMap::new(),
            // viewable: HashMap::new(),
        }
    }

    // pub fn get_viewable() {
        
    // }

    pub fn get_moves(&mut self, turn: i64) -> Option<Vec<Move>> {
        self.moves.remove(&turn)
    }

    pub fn set_move(&mut self, turn: i64, p_move: Move) {
        if let Some(data) = self.moves.get_mut(&turn) {
            data.push(p_move);
        } else {
            self.moves.insert(turn, vec!(p_move));
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct King {
    pub color: Color,
    pub troops: i64,
}

impl King {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            troops: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Field {
    pub color: Color,
    pub troops: i64,
}

impl Field {
    pub fn new(color: Color) -> Self {
        Field {
            color,
            troops: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Tile {
    King(King),
    Field(Field),
}

struct Map {
    width: usize, 
    height: usize, 
    pub tiles: Vec<Vec<Tile>>, // WARN: remove pub
    changes: HashMap<(usize, usize), Tile>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = vec![vec![Tile::Field(Field::new(Color::Empty)); width]; height];
        tiles[0][2] = Tile::King(King::new(Color::Blue));
        tiles[5][7] = Tile::King(King::new(Color::Green));
        let changes = HashMap::new();

        Self {
            width,
            height,
            tiles,
            changes,
        }
    }

    pub fn get_changes(&mut self) -> HashMap<(usize, usize), Tile> {
        let changes = self.changes.clone(); // PROB: very ugly
        self.changes = HashMap::new();
        changes
    }

    pub fn make_move(&mut self, p_move: Move) {
        match p_move {
            Move::Step((fx, fy), (tx, ty), ratio) => {
                let color;
                match &mut self.tiles[fy][fx] { // later with trait
                    Tile::King(obj) => {
                        color = obj.color.clone();
                    },
                    _ => panic!("This tile type can not make a step"),
                }
                match &mut self.tiles[ty][tx] { // later with trait
                    Tile::King(obj) => {
                        obj.color = color;
                    },
                    Tile::Field(obj) => {
                        obj.color = color;
                    },
                    _ => panic!("This tile type can not make a step"),
                }
                let tile = self.tiles[tx][ty].clone();
                self.changes.insert((tx, ty), tile);
            }
        }
    }
}

struct State {
    turn: i64,
    map: Map,
    players: HashMap<i64, Player>,
}

impl State {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            turn: 0,
            map: Map::new(width, height),
            players: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        unimplemented!();
    }

    pub fn get_tiles(&self) -> Vec<Vec<Tile>> {
        self.map.tiles.clone() // QUES: how better?
    }

    pub fn get_changes(&mut self) -> HashMap<(usize, usize), Tile> {
        self.map.get_changes()
    }

    pub fn make_move(&mut self, p_move: Move) {
        self.map.make_move(p_move);
    }

    // pub fn get_serialized_tiles(&self) -> std::result::Result<std::vec::Vec<u8>, serde_json::error::Error> {
    //     serde_json::to_vec(&DivGameResponse::Running(DivGameRunningResponse::State(&self.map.tiles)))
    // }

    // pub fn 
}