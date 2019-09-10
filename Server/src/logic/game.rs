use std::sync::{Arc, Mutex};

use rand::{self, Rng};

use crate::logic::client;
use crate::web_socket;

use std::thread;
use std::time::Duration;

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

pub struct GameData {
    clients: Arc<Mutex<Vec<client::Client>>>,
    state: State,
    websocket: Option<web_socket::WebSocket>,
    ip: String,
    port: String,
    connected: u16,
    ready: u16,
    running: bool,
    started: bool,
}

impl GameData {
    pub fn new(ip: String, port: String) -> GameData {
        GameData{clients: Arc::new(Mutex::new(Vec::new())), state: State::new(10, 10), websocket: None, ip, port, connected: 0, ready: 0, running: false, started: false}
    }

    pub fn connect(&mut self, client: client::Client) {
        self.connected += 1;
        self.clients.lock().unwrap().push(client); // create a player; // error if full?;
    }

    // all of these needed?
    pub fn get_ip(&mut self) -> String {
        self.ip.clone()
    }

    pub fn get_port(&mut self) -> String {
        self.port.clone()
    }

    pub fn get_clients(&mut self) -> Arc<Mutex<Vec<client::Client>>> {
        self.clients.clone()
    }

    pub fn get_ready(&mut self) -> u16 {
        self.ready.clone()
    }

    pub fn set_ready(&mut self, ready: u16) {
        self.ready = ready;
    }

    pub fn set_websocket(&mut self, websocket: Option<web_socket::WebSocket>) {
        self.websocket = websocket;
    }

    pub fn get_connected(&mut self) -> u16 {
        self.connected.clone()
    }

    // pub fn set_connected(&mut self, connected: u16) {
    //     self.connected = connected;
    // }

    pub fn get_started(&mut self) -> bool {
        self.started
    }

    pub fn set_started(&mut self, started: bool) {
        self.started = started;
    }

    pub fn receive_message(&mut self) -> Option<String> {
        for client in self.clients.lock().unwrap().iter() { // iter vs normal???
            if let Some(message) = client.try_recv() {
                return Some(message);
            }
        }

        None
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

        self.broadcast(&message);        
    }

    pub fn broadcast(&mut self, message: &str) { // Vec<u8>
        for client in self.clients.lock().unwrap().iter() {
            client.send(&message);
        }
    }

    pub fn match_message(&mut self, message: String) { // set client => message struct with ids and stuff
        if message == "ready" {
            self.ready += 1; // check this per client

            if self.connected / 2 + 1 > self.ready {
                return;
            }

            self.started = true;
            self.broadcast("game_started");
            
            // let clients = game_data.lock().unwrap().get_clients();
            // let mut rng = rand::thread_rng();

            // {
            //     let clients = clients.lock().unwrap();
            //     for i in 0..clients.clients.len() {
            //         let x = rng.gen_range(0,10);
            //         let y = rng.gen_range(0,10);

            //         let player_color = match i {
            //             0 => PlayerColor::Red,
            //             1 => PlayerColor::Green,
            //             2 => PlayerColor::Blue,
            //             _ => PlayerColor::Empty,
            //         };

            //         let field = Field::new(FieldType::King, player_color);
            //         // self.update_single_state(x, y, field);
            //         game_data.lock().unwrap().update_single_state(x, y, field);
            //     }
            // }

            // for y in 0..10 {
            //     for x in 0..10 {
            //         // self.send_single_state(x, y);
            //         game_data.lock().unwrap().send_single_state(x, y);
            //     }
            // }
        }
    }
}

pub struct Game {
    game_data: Arc<Mutex<GameData>>, //remove pub
}

impl Game {
    pub fn new(ip: String, port: String) -> Game {
        Game{game_data: Arc::new(Mutex::new(GameData::new(ip, port)))}
    }

    pub fn start(&mut self) { //&mut self, ip: String, port: String) { // put in two functinos, one to start the game and one for the player
        let game_data = self.game_data.clone();

        if game_data.lock().unwrap().running {
            return;
        }
        game_data.lock().unwrap().running = true;


        let mut websocket = web_socket::WebSocket::new(self.get_ip(), self.get_port());
        websocket.start(self.game_data.clone());
        game_data.lock().unwrap().set_websocket(Some(websocket));

        thread::spawn(move || { // turn this into game loop => receive everything => alter state => send state;
            loop {
                thread::sleep(Duration::from_millis(1));
                
                loop {
                    let message: String;
                    if let Some(res) = game_data.lock().unwrap().receive_message() {
                        println!("message: {}", &res); // {} vs {:?}
                        message = res;
                    } else {
                        break;
                    }

                    game_data.lock().unwrap().match_message(message);
                }
            }
        });
    }

    pub fn get_ip(&mut self) -> String {
        self.game_data.lock().unwrap().get_ip()
    }

    pub fn get_port(&mut self) -> String {
        self.game_data.lock().unwrap().get_port()
    }
}