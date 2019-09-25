use std::{str, time, thread};
use std::sync::{Arc, Mutex};

use std::collections::HashMap;
use log;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::websocket_server::ws_connection::WSConnection;
use crate::websocket_server::server::WebSocketServer;

use crate::logic::trait_game::Game;

use crate::connection::trait_handle_message::HandleMessage;
use crate::connection::trait_handle_new_connection::HandleNewConnection;
use crate::connection::trait_connection::Connection;


pub struct NormalDivGame { // QUES: IDEA: common sub type for NormalDivGame & Api Server, a lot of data is the same
    connections: Arc<Mutex<NorDivGameConnectionHandler>>,
    ws_server: WebSocketServer,
    ip: String,
    port: String,
    running: bool,
}

impl NormalDivGame { // QUES: IDEA: trait server ???
    pub fn new(ip: &str, port: &str) -> NormalDivGame {
        log::info!("new NormalDivGame created on {}:{}", ip, port);
        NormalDivGame{connections: Arc::new(Mutex::new(NorDivGameConnectionHandler::new())), ws_server: WebSocketServer::new(ip, port), ip: ip.to_string(), port: port.to_string(), running: false}
    }
}

impl Game for NormalDivGame {
    fn start_server(&mut self) { // PROB: return handle
        if self.running {
            return;
        }
        self.running = true;

        log::info!("starting NormalDivGame WSServer on {}:{}", &self.ip, &self.port);
        self.ws_server.start(self.connections.clone());
        self.start_game();
    }

    fn start_game(&mut self) { // QUES: second running check?
        let connections = self.connections.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(time::Duration::from_millis(1));
                
                loop {
                    let message: Message;
                    if let Some(res) = connections.lock().unwrap().receive_message() {
                        let content = res.content.clone();
                        log::debug!("{}", str::from_utf8(&content).unwrap()); // QUES: error handling? // WARN: QUES: too ineficient???
                        message = res;
                    } else {
                        break; // PROB: error handling?
                    }

                    connections.lock().unwrap().handle_message(message); // QUES: bytes and string?? // QUES: two completely different
                }
            }
        });
    }

    fn update(&mut self) {
        log::debug!("NormalDivGame instance updated");
    }
}

struct NorDivGameConnectionHandler {
    connections: HashMap<i64, Arc<Mutex<NorDivGameClient>>>, // PROB: NorDivGameClients hanging around that are no more on the list
    available_ids: Vec<i64>,
    highest_id: i64,
}

impl NorDivGameConnectionHandler { // PROB: QUES: remove clients that reload page
    fn new() -> NorDivGameConnectionHandler {
        log::info!("new NorDivGameConnectionHandler was created");
        NorDivGameConnectionHandler{connections: HashMap::new(), available_ids: Vec::new(), highest_id: 0,}
    }

    pub fn receive_message<'a>(&mut self) -> Option<Message> { // IDEA: semaphores with producer consumer
        for (id, org_client) in (&self.connections).iter() { // iter vs normal??? // PROB: keep track of order => not every time the same one
            let client = org_client.clone();
            let message_res = client.lock().unwrap().connection.try_recv();
            if let Some(message) = message_res {
                log::info!("NorDivGameConnectionHandler received a message");
                return Some(Message::new(client, message));
            }
        }

        None
    }

    fn disconnect_client(&mut self, id: i64) {
        if let Some(client) = self.connections.remove(&id) {
            client.lock().unwrap().close_connection();
            self.available_ids.push(id);
        } else {
            panic!("Client does not exist"); // PROB: nice handling
        }
    }
}

impl HandleNewConnection<WSConnection> for NorDivGameConnectionHandler { // QUES: do it directly in API server? send connection over stream? callback function?
    fn handle_new_connection(&mut self, connection: WSConnection) {
        let id: i64;
        if let Some(unused_id) = self.available_ids.pop() {
            id = unused_id;
        } else {
            id = self.highest_id;
            self.highest_id += 1;
        }
        log::debug!("APIServer is handling a new connection with id: {}", &id);
        let client = Arc::new(Mutex::new(NorDivGameClient::new(id, connection)));
        self.connections.insert(id, client);
    }
}

impl HandleMessage<Message> for NorDivGameConnectionHandler {
    fn handle_message(&mut self, message: Message) {
        log::debug!("NorDivGameConnectionHandler is handling a message");
        for (id, client) in (&self.connections).iter() {
            println!("{}", id);
        }
        let content = str::from_utf8(&message.content).unwrap(); // PROB: error handling
        if let Ok(api_request) = serde_json::from_str::<NorDivGameRequest>(content) {
            match api_request {
                NorDivGameRequest::JoinDivGameNormal => {
                    log::info!("client joined a normal div game");
                    message.sender.lock().unwrap().connection.send(serde_json::to_vec(&NorDivGameResponse::JoinGame("some_id".to_string())).unwrap()); // PROB: error handling // QUES: efficiency?
                    let id = message.sender.lock().unwrap().id; // QUES: two times lock bad?
                    self.disconnect_client(id);
                },
                NorDivGameRequest::GetOpenLobbies => {
                    log::debug!("client asked for open lobbies");
                    message.sender.lock().unwrap().connection.send(serde_json::to_vec(&NorDivGameResponse::OpenLobbies(vec!(r#"{"id": "some_id", "max_players": 8, "current_players": 4}"#.to_string()))).unwrap())
                },
                NorDivGameRequest::GetRunningGames => {
                    log::debug!("client asked for open lobbies");
                    message.sender.lock().unwrap().connection.send(serde_json::to_vec(&NorDivGameResponse::RunningGames(vec!(r#"{"id": "some_id", "players_start": 8, "current_players": 4, "ticks": 243}"#.to_string()))).unwrap())
                },
                _ => {
                    log::warn!("invalid NorDivGameRequest send to APIServer");
                    message.sender.lock().unwrap().connection.send(serde_json::to_vec(&NorDivGameResponse::InvalidRequest).unwrap());
                },
            }
        } else { // Prob: QUES: WARN: differentiate between invalid json and invalid request
            log::warn!("An error occured when parsing message");
            message.sender.lock().unwrap().connection.send(serde_json::to_vec(&NorDivGameResponse::InvalidJson).unwrap()); // PROB: error handling // QUES: efficiency?
        }
    }
}

struct NorDivGameClient {
    id: i64,
    connection: WSConnection,
}

impl NorDivGameClient {
    fn new(id: i64, connection: WSConnection) -> NorDivGameClient {
        log::info!("new API client was created");
        NorDivGameClient {id, connection,}
    }

    pub fn close_connection(&mut self) {
        log::info!("close connection from NorDivGameClient {}", &self.id);
        self.connection.close(); // WARN: trait in WSConnection that handles close
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum NorDivGameRequest {
    JoinDivGameNormal,
    GetRunningGames,
    GetOpenLobbies,
}

#[derive(Serialize, Deserialize, Debug)]
enum NorDivGameResponse {
    InvalidJson,
    InvalidRequest,
    JoinGame(String),
    OpenLobbies(Vec<String>), // TODO: Vec<GameMetaData>
    RunningGames(Vec<String>), // TODO: Vec<GameMetaData>
}

struct Message {
    sender: Arc<Mutex<NorDivGameClient>>, // QUES: what exactly does 'static do here?
    content: Vec<u8>,
}

impl Message {
    fn new(sender: Arc<Mutex<NorDivGameClient>>, content: Vec<u8>) -> Message { // QUES: what exactly does 'static do here?
        log::info!("new Message was created"); // QUES: better identifier
        Message{sender, content}
    }
}