use std::collections::{HashMap, HashSet};
use std::str;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use serde_json;

use crate::logic::bucket_server::{BaseBucketData, BaseBucketMessage, BaseConnectionHandler};
use crate::logic::Bucket;

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
    state: State,
    bucket_data: BaseBucketData,
    bucket_running: bool,
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
            bucket_state: DivGameBucketState::new(),
            state: State::new(10, 10),
            bucket_data,
            bucket_running: false,
        }
    }
}

impl Bucket for DivGameBucket {
    fn start(&mut self) {
        log::info!("DivGameBucket started");
        self.bucket_running = true;
    }

    fn stop(&mut self) {
        log::info!("DivGameBucket stoped");
        self.bucket_running = false;
    }

    fn update(&mut self) {
        if !self.bucket_state.running {
            return;
        }
        self.state.update();
        if !self.state.changes_exist() {
            return;
        }
        let changes = self.state.get_changes_serialized().unwrap();
        self.connection_handler.lock().unwrap().broadcast(
            changes
        );
    }

    fn handle_message(&mut self, mut message: BaseBucketMessage) {
        //}, bucket_manager: Arc<Mutex<BaseBucketManager>>) {
        let msg = message.get_content();
        log::info!(
            "DivGameBucket received a message: {}",
            str::from_utf8(&msg).unwrap()
        );
        let client = message.get_client();
        let client_id = client.lock().unwrap().get_id();

        if let Ok(api_request) = serde_json::from_slice::<DivGameRequest>(&msg) {
            match api_request {
                DivGameRequest::Lobby(lobby_request) => {
                    if self.bucket_state.running {
                        log::warn!(
                            "DivGameBucket received a lobby request although game is running"
                        );
                        return;
                    }
                    if let DivGameLobbyRequest::Join = lobby_request {}
                    else if !self.state.player_exists(&client_id) { // WARN: QUES: check if state exists and stuff
                        log::warn!("only a user that joined can call this instruction");
                        return;
                    }
                    match lobby_request {
                        DivGameLobbyRequest::Join => {
                            let color = match client_id {
                                0 => Color::Blue,
                                1 => Color::Red,
                                _ => Color::Green,
                            };
                            self.bucket_state.clients += 1;
                            self.state.players.insert(client_id, Player::new(color));
                        },
                        DivGameLobbyRequest::Ready => {
                            let ready = client.lock().unwrap().get_ready();
                            if ready {
                                return;
                            }
                            client.lock().unwrap().set_ready(true);
                            log::debug!("client is ready");

                            self.bucket_state.clients =
                                self.connection_handler.lock().unwrap().connections.len() as u64;
                            self.bucket_state.ready += 1;
                            // if self.state.clients > 1 && self.state.ready * 3 > self.state.clients * 2 {
                            if self.bucket_state.ready * 3 > self.bucket_state.clients * 2 {
                                log::info!("starting game");
                                self.bucket_state.running = true;
                                let id = self.bucket_data.get_id();
                                self.bucket_manager.lock().unwrap().start_lobby(id);
                                // let mut state = State::new(10, 10); // update state with settings

                                let tiles = self.state.get_tiles();
                                self.connection_handler.lock().unwrap().broadcast(
                                    serde_json::to_vec(&DivGameResponse::Lobby(
                                        DivGameLobbyResponse::StartGame(tiles),
                                    ))
                                    .unwrap(),
                                ); // QEUS: WARN: a lot of useless clone
                                self.state.make_move(Move::Step((2, 0), (2, 1), 0.5));
                                
                                let changes = self.state
                                        .get_changes_serialized()
                                        .expect("Could not serialize changes in state");
                                self.connection_handler.lock().unwrap().broadcast(
                                    changes
                                ); // a lot of useless clone
                            }
                        },
                        DivGameLobbyRequest::NotReady => {
                            let ready = client.lock().unwrap().get_ready();
                            if !ready {
                                return;
                            }
                            client.lock().unwrap().set_ready(false);
                            log::debug!("client is not ready");

                            // self.bucket_state.clients =
                            //     self.connection_handler.lock().unwrap().connections.len() as u64;
                            self.bucket_state.ready -= 1;
                        },
                        _ => {
                            log::warn!("invalid DivGame Loby request");
                            client.lock().unwrap().send(
                                serde_json::to_vec(&DivGameResponse::InvalidRequest).unwrap(),
                            );
                        }
                    }
                }
                DivGameRequest::Running(request) => match request {
                    DivGameRunningRequest::Move(p_move) => {
                        self.state
                            .add_move(client_id, p_move);
                    }
                    _ => {
                        log::warn!("invalid DivGame Loby request");
                        client
                            .lock()
                            .unwrap()
                            .send(serde_json::to_vec(&DivGameResponse::InvalidRequest).unwrap());
                    }
                },
                _ => {
                    log::warn!("invalid APIRequest send to APIServer");
                    client
                        .lock()
                        .unwrap()
                        .send(serde_json::to_vec(&DivGameResponse::InvalidRequest).unwrap());
                }
            }
        } else {
            // Prob: QUES: WARN: differentiate between invalid json and invalid request
            log::warn!("An error occured when parsing message");
            client
                .lock()
                .unwrap()
                .send(serde_json::to_vec(&DivGameResponse::InvalidJson).unwrap()); // PROB: error handling // QUES: efficiency?
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum DivGameRunningRequest {
    Move(Move),
}

#[derive(Serialize, Deserialize, Debug)]
enum DivGameLobbyRequest {
    Join,
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
    StateUpdate(Vec<(usize, usize, Tile)>),
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
            self.moves.insert(turn, vec![p_move]);
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
        Self { color, troops: 0 }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Field {
    pub color: Color,
    pub troops: i64,
}

impl Field {
    pub fn new(color: Color) -> Self {
        Field { color, troops: 0 }
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
    changes: HashSet<(usize, usize)>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = vec![vec![Tile::Field(Field::new(Color::Empty)); width]; height];
        tiles[0][2] = Tile::King(King::new(Color::Blue));
        tiles[5][7] = Tile::King(King::new(Color::Green));
        let changes = HashSet::new();

        Self {
            width,
            height,
            tiles,
            changes,
        }
    }

    pub fn changes_exist(&self) -> bool {
        !self.changes.is_empty()
    }

    pub fn get_changes(&mut self) -> Vec<(usize, usize, Tile)> {
        // WARN: expensive
        let mut res = Vec::new();
        for change in &self.changes {
            let (x, y) = change;
            res.push((*x, *y, self.tiles[*y][*x].clone()));
        }
        self.changes.clear();
        res
    }

    pub fn get_changes_serialized(
        &mut self,
    ) -> std::result::Result<Vec<u8>, serde_json::error::Error> {
        serde_json::to_vec(&DivGameResponse::Running(
            DivGameRunningResponse::StateUpdate(self.get_changes()),
        ))
    }

    pub fn get_state_serialized(&self) -> std::result::Result<Vec<u8>, serde_json::error::Error> {
        serde_json::to_vec(&DivGameResponse::Running(DivGameRunningResponse::State(
            self.tiles.clone(),
        )))
    }

    pub fn make_move(&mut self, p_move: Move) {
        match p_move {
            Move::Step((fx, fy), (tx, ty), ratio) => {
                let color;
                match &mut self.tiles[fy][fx] {
                    // later with trait
                    Tile::King(obj) => {
                        color = obj.color.clone();
                    },
                    Tile::Field(obj) => {
                        color = obj.color.clone();
                    },
                    _ => panic!("This tile type can not make a step"),
                }
                match &mut self.tiles[ty][tx] {
                    // later with trait
                    Tile::King(obj) => {
                        obj.color = color;
                    }
                    Tile::Field(obj) => {
                        obj.color = color;
                    }
                    _ => panic!("This tile type can not make a step"),
                }
                let tile = self.tiles[tx][ty].clone();
                // self.changes.insert((tx, ty), tile);
                self.changes.insert((tx, ty));
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
        let mut collected_moves: Vec<(i64, Move)> = Vec::new();
        for (id, player) in &mut self.players {
            let p_moves = player.get_moves(self.turn);
            if let Some(moves) = p_moves {
                for p_move in moves {
                    collected_moves.push((*id, p_move));
                }
            }
        }
        for (_id, p_move) in collected_moves {
            // WARN: validate moves, does id belong to player, etc
            self.make_move(p_move);
        }
        self.turn += 1;
    }

    pub fn get_tiles(&self) -> Vec<Vec<Tile>> {
        self.map.tiles.clone() // QUES: how better?
    }

    pub fn changes_exist(&self) -> bool {
        self.map.changes_exist()
    }

    pub fn get_state_serialized(&self) -> std::result::Result<Vec<u8>, serde_json::error::Error> {
        self.map.get_state_serialized()
    }

    pub fn get_changes_serialized(
        &mut self,
    ) -> std::result::Result<Vec<u8>, serde_json::error::Error> {
        self.map.get_changes_serialized()
    }

    pub fn get_changes(&mut self) -> Vec<(usize, usize, Tile)> {
        self.map.get_changes()
    }

    pub fn add_move(&mut self, id: i64, p_move: Move) {
        let turn = self.turn;
        match &mut self.players.get_mut(&id) {
            Some(player) => {
                player.set_move(turn + 1, p_move);
            }
            None => {
                log::warn!("no player found with that id: {}", id);
            }
        }
    }

    fn make_move(&mut self, p_move: Move) {
        self.map.make_move(p_move);
    }

    pub fn player_exists(&self, id: &i64) -> bool {
        self.players.contains_key(id)
    }

    // pub fn get_serialized_tiles(&self) -> std::result::Result<std::vec::Vec<u8>, serde_json::error::Error> {
    //     serde_json::to_vec(&DivGameResponse::Running(DivGameRunningResponse::State(&self.map.tiles)))
    // }

    // pub fn
}
