use std::{str, sync::{Arc, Mutex}};

use serde::{Deserialize, Serialize};
use serde_json;

extern crate bucketer;
use bucketer::bucket::{Bucket, BucketMessage, ConnectionHandler};

pub struct TicTacToe {
    state: [[Option<bool>; 3]; 3],
    turn: bool,
    running: bool,
    connection_handler: Arc<Mutex<ConnectionHandler>>,
    players: Vec<u64>,
}

impl TicTacToe {
    pub fn new(connection_handler: Arc<Mutex<ConnectionHandler>>) -> Self {
        Self {
            state: [[None; 3]; 3],
            turn: false,
            running: false,
            connection_handler,
            players: Vec::new(),
        }
    }
}

impl Bucket for TicTacToe {
    fn start(&mut self) {
        unimplemented!();
    }

    fn stop(&mut self) {
        unimplemented!();
    }

    fn handle_message(&mut self, message: BucketMessage) {
        let msg = message.get_content();
        log::info!(
            "DivGameBucket received a message: {}",
            str::from_utf8(&msg).unwrap()
        );
        let client = message.get_sender();
        let client_id = client.lock().unwrap().get_id();

        if let Ok(api_request) = serde_json::from_slice::<Request>(&msg) {
            match api_request {
                Request::Ready => {
                    match self.players.len() {
                        0 => self.players.push(client_id),
                        1 => {
                            if self.players[0] == client_id {
                                return;
                            }
                            self.players.push(client_id);
                            self.running = true;
                        },
                        _ => {
                            return;
                        }
                    }
                },
                Request::Place(x, y) => {
                    if !self.running || !self.state[y as usize][x as usize].is_none() {
                        return;
                    }
                    if (self.turn && self.players[0] != client_id) || (!self.turn && self.players[1] != client_id) {
                        return;
                    }
                    self.state[y as usize][x as usize] = Some(self.turn);
                    let state = serde_json::to_vec(&Response::State(self.state.clone())).expect("serializing failed");
                    self.connection_handler.lock().unwrap().broadcast(state).unwrap();
                    
                    let mut winner = None;
                    for i in 0..2 {
                        let field = match self.state[i][0] {
                            Some(field) => field,
                            _ => continue,
                        };
                        match self.state[i][1] {
                            Some(f) => if f != field {continue;},
                            _ => continue,
                        }
                        match self.state[i][2] {
                            Some(f) => if f == field {winner = Some(self.turn)},
                            _ => continue,
                        }
                    }

                    for i in 0..2 {
                        let field = match self.state[0][i] {
                            Some(field) => field,
                            _ => continue,
                        };
                        match self.state[1][i] {
                            Some(f) => if f != field {continue;},
                            _ => continue,
                        }
                        match self.state[2][i] {
                            Some(f) => if f == field {winner = Some(self.turn)},
                            _ => continue,
                        }
                    }

                    let part = self.state[0][0];
                    if part.is_some() {
                        if self.state[0][0] == self.state[1][1] && self.state[1][1] == self.state[2][2] {
                            winner = Some(self.turn);
                        }
                    }
                    
                    let part = self.state[2][0];
                    if part.is_some() {
                        if self.state[2][0] == self.state[1][1] && self.state[1][1] == self.state[0][2] {
                            winner = Some(self.turn);
                        }
                    }

                    if let Some(winner) = winner {
                        let wid = self.players[if winner {0} else {1}];
                        let lid = self.players[if winner {1} else {0}];
                        let w_message = serde_json::to_vec(&Response::Win).expect("serializing failed");
                        let l_message = serde_json::to_vec(&Response::Defeat).expect("serializing failed");
                        self.connection_handler.lock().unwrap().connections[&wid].lock().unwrap().send(w_message).expect("sending failed");
                        self.connection_handler.lock().unwrap().connections[&lid].lock().unwrap().send(l_message).expect("sending failed");
                    }

                    self.turn = !self.turn;
                }
            }
        } else {
            log::error!("unknown...\t{:?}", str::from_utf8(&msg));
        }
    }
}

#[derive(Deserialize, Debug)]
enum Request {
    Ready,
    Place(u8, u8),
}

#[derive(Serialize)]
enum Response {
    State([[Option<bool>; 3]; 3]),
    Win,
    Defeat,
}
