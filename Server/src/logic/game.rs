use std::sync::{Arc, Mutex};

use rand::{self, Rng};

use crate::logic::client;
use crate::web_socket;

enum FieldType {
    Ground,
    King,
}

enum PlayerColor {
    Empty,
    Red,
    Green,
    Blue,
}

pub struct Field {
    field_type: FieldType,
    player_color: PlayerColor,
}

impl Field {
    fn new(field_type: FieldType, player_color: PlayerColor) -> Field {
        Field{field_type, player_color}
    }
}

struct State {
    x_size: i64,
    y_size: i64,
    map: Vec<Vec<Field>>,
}

impl State {
    fn new(x_size: i64, y_size: i64) -> State {
        let mut map = Vec::new();
        for _ in 0..y_size {
            let mut col = Vec::new();
            for _ in 0..x_size {
                col.push(Field::new(FieldType::Ground, PlayerColor::Empty));
            }
            map.push(col);
        }
        State{x_size, y_size, map}
    }
}

pub trait SecureAdd<T> {
    fn add(&mut self, object: T);
}

pub struct SecureList {
    pub clients: Vec<client::Client>,
}

pub struct Game {
    pub clients: Arc<Mutex<SecureList>>, // make private
    state: State,
    websocket: Option<web_socket::WebSocket>,
    pub ip: String,
    pub port: String,
    connected: u16,
    wanna_start: u16,
    running: bool,
}

impl Game {
    pub fn new(ip: String, port: String) -> Game {
        Game{clients: Arc::new(Mutex::new(SecureList{clients: Vec::new()})), state: State::new(10, 10), websocket: None, ip, port, connected: 2, wanna_start: 0, running: false}
    }

    pub fn start(&mut self) { //&mut self, ip: String, port: String) { // put in two functinos, one to start the game and one for the player
        // self.ip = ip;
        // self.port = port;
        if self.running == false {
            self.running = true;
            let mut websocket = web_socket::WebSocket::new(&self.ip, &self.port);
            websocket.start(self.clients.clone());
            self.websocket = Some(websocket);
        }
        if self.connected > 0 {
            self.connected -= 1;
            return;
        }

        let clients = self.clients.clone();
        let mut rng = rand::thread_rng();

        {
            let clients = clients.lock().unwrap();
            for i in 0..clients.clients.len() {
                let x = rng.gen_range(0,10);
                let y = rng.gen_range(0,10);

                let player_color = match i {
                    0 => PlayerColor::Red,
                    1 => PlayerColor::Green,
                    2 => PlayerColor::Blue,
                    _ => PlayerColor::Empty,
                };

                let field = Field::new(FieldType::King, player_color);
                self.update_single_state(x, y, field);
            }
        }

        for y in 0..10 {
            for x in 0..10 {
                self.send_single_state(x, y);
            }
        }
    }

    pub fn update_single_state(&mut self, x: i64, y:i64, field: Field) {
        self.state.map[y as usize][x as usize] = field;
    }

    pub fn send_single_state(&mut self, x: i64, y: i64) {
        let field_type = match self.state.map[y as usize][x as usize].field_type {
            FieldType::Ground => 0,
            FieldType::King => 2,
        };
        let player_color = match self.state.map[y as usize][x as usize].player_color {
            PlayerColor::Empty => 0,
            PlayerColor::Red => 1,
            PlayerColor::Green => 2,
            PlayerColor::Blue => 3,
        };

        let message = format!("{}{}{}{}0311", x, y, player_color, field_type);

        let clients = self.clients.lock().unwrap();
        for client in &clients.clients {
            client.send(&message);
        }
    }
}

impl SecureAdd<client::Client> for SecureList {
    fn add(&mut self, client: client::Client) {
        self.clients.push(client);
    }
}