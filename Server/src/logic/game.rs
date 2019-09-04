use crate::logic::client;

use std::sync::{Arc, Mutex};
use std::thread;

use rand::{self, Rng};

enum FieldType {
    Ground,
    King,
}

struct Field {
    field_type: FieldType,
}

impl Field {
    fn new(field_type: FieldType) -> Field {
        Field{field_type}
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
                col.push(Field::new(FieldType::Ground));
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
}

impl Game {
    pub fn new() -> Game {
        Game{clients: Arc::new(Mutex::new(SecureList{clients: Vec::new()})), state: State::new(10, 10)}
    }

    pub fn start(&mut self) {
        let clients = self.clients.clone();
        // let _game_handle = thread::spawn(move || -> std::io::Result<()> {
            let mut rng = rand::thread_rng();

            {
                let clients = clients.lock().unwrap();
                for client in &clients.clients {
                    let x = rng.gen_range(0,10);
                    let y = rng.gen_range(0,10);

                    let field = Field::new(FieldType::King);
                    self.update_single_state(x, y, field);
                    
                    // let message = format!("{}{}020311", x, y);
                    // client.send(&message);
                }
            }

            // for client in &clients.clients {
                for y in 0..10 {
                    for x in 0..10 {
                        println!("sending");
                        self.send_single_state(x, y);
                        println!("sent");
                    }
                }
            // }

            // Ok(())
        // });
    }

    pub fn update_single_state(&mut self, x: i64, y:i64, field: Field) {
        self.state.map[y as usize][x as usize] = field;
    }

    pub fn send_single_state(&mut self, x: i64, y: i64) {
        let field_type = match self.state.map[y as usize][x as usize].field_type {
            FieldType::Ground => 0,
            FieldType::King => 2,
        };

        let message = format!("{}{}0{}0311", x, y, field_type);
        println!("message: {}", &message);

        let clients = self.clients.lock().unwrap();
        for client in &clients.clients {
            client.send(&message);
        }
    }
}

impl SecureAdd<client::Client> for SecureList {
    fn add(&mut self, client: client::Client) {
        println!("hello");
        self.clients.push(client);
    }
}