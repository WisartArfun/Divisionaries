use std::{str, time, thread};
use std::sync::{Arc, Mutex};

use std::collections::HashMap;
use log;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::websocket_server::ws_connection::WSConnection;
use crate::websocket_server::server::WebSocketServer;

use crate::connection::trait_handle_new_connection::HandleNewConnection;
use crate::connection::trait_connection::Connection;
use crate::connection::trait_handle_message::HandleMessage;

pub struct APIServer {
    connections: Arc<Mutex<WSConnectionHandler>>,
    ws_server: WebSocketServer,
    ip: String,
    port: String,
    running: bool,
}

impl APIServer {
    pub fn new(ip: &str, port: &str) -> APIServer {
        log::info!("new APIServer created on {}:{}", ip, port);
        APIServer{connections: Arc::new(Mutex::new(WSConnectionHandler::new())), ws_server: WebSocketServer::new(ip, port), ip: ip.to_string(), port: port.to_string(), running: false}
    }

    pub fn start(&mut self) { // PROB: return handle
        if self.running {
            return;
        }
        self.running = true;

        log::info!("starting server on {}:{}", &self.ip, &self.port);
        self.ws_server.start(self.connections.clone());
        self.start_api_loop();
    }

    pub fn start_api_loop(&mut self) { // QUES: second running check?
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
}

struct WSConnectionHandler {
    connections: HashMap<i64, Arc<Mutex<APIClient>>>, // PROB: apiclients hanging around that are no more on the list
    available_ids: Vec<i64>,
    highest_id: i64,
}

impl HandleNewConnection<WSConnection> for WSConnectionHandler { // QUES: do it directly in API server? send connection over stream? callback function?
    fn handle_new_connection(&mut self, connection: WSConnection) {
        let id: i64;
        if let Some(unused_id) = self.available_ids.pop() {
            id = unused_id;
        } else {
            id = self.highest_id;
            self.highest_id += 1;
        }
        log::debug!("APIServer is handling a new connection with id: {}", &id);
        let api_client = Arc::new(Mutex::new(APIClient::new(id, connection)));
        self.connections.insert(id, api_client);
    }
}

impl HandleMessage<Message> for WSConnectionHandler {
    fn handle_message(&mut self, message: Message) {
        log::debug!("WSConnectionHandler is handling a message");
        for (id, client) in (&self.connections).iter() {
            println!("{}", id);
        }
        let content = str::from_utf8(&message.content).unwrap(); // PROB: error handling
        if let Ok(api_request) = serde_json::from_str::<APIRequest>(content) {
            match api_request {
                APIRequest::JoinDivGameNormal => {
                    log::info!("client joined a normal div game");
                    message.sender.lock().unwrap().connection.send(serde_json::to_vec(&APIResponse::JoinGame("some_id".to_string())).unwrap()); // PROB: error handling // QUES: efficiency?
                    let id = message.sender.lock().unwrap().id; // QUES: two times lock bad?
                    self.disconnect_client(id);
                },
                APIRequest::GetOpenLobbies => {
                    log::debug!("client asked for open lobbies");
                    message.sender.lock().unwrap().connection.send(serde_json::to_vec(&APIResponse::OpenLobbies(vec!(r#"{"id": "some_id", "max_players": 8, "current_players": 4}"#.to_string()))).unwrap())
                },
                APIRequest::GetRunningGames => {
                    log::debug!("client asked for open lobbies");
                    message.sender.lock().unwrap().connection.send(serde_json::to_vec(&APIResponse::RunningGames(vec!(r#"{"id": "some_id", "players_start": 8, "current_players": 4, "ticks": 243}"#.to_string()))).unwrap())
                },
                _ => {
                    log::warn!("invalid APIRequest send to APIServer");
                    message.sender.lock().unwrap().connection.send(serde_json::to_vec(&APIResponse::InvalidRequest).unwrap());
                },
            }
        } else { // Prob: QUES: WARN: differentiate between invalid json and invalid request
            log::warn!("An error occured when parsing message");
            message.sender.lock().unwrap().connection.send(serde_json::to_vec(&APIResponse::InvalidJson).unwrap()); // PROB: error handling // QUES: efficiency?
        }
    }
}

impl WSConnectionHandler { // PROB: QUES: remove clients that reload page
    fn new() -> WSConnectionHandler {
        log::info!("new WSConnectionHandler was created");
        WSConnectionHandler{connections: HashMap::new(), available_ids: Vec::new(), highest_id: 0,}
    }

    pub fn receive_message<'a>(&mut self) -> Option<Message> { // IDEA: semaphores with producer consumer
        for (id, org_client) in (&self.connections).iter() { // iter vs normal??? // PROB: keep track of order => not every time the same one
            let client = org_client.clone();
            let message_res = client.lock().unwrap().connection.try_recv();
            if let Some(message) = message_res {
                log::info!("WSConnectionHandler received a message");
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

struct APIClient {
    id: i64,
    connection: WSConnection,
}

impl APIClient {
    fn new(id: i64, connection: WSConnection) -> APIClient {
        log::info!("new API client was created");
        APIClient {id, connection,}
    }

    pub fn close_connection(&mut self) {
        log::info!("close connection from APIClient {}", &self.id);
        self.connection.close(); // WARN: trait in WSConnection that handles close
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum APIRequest {
    JoinDivGameNormal,
    GetRunningGames,
    GetOpenLobbies,
}

#[derive(Serialize, Deserialize, Debug)]
enum APIResponse {
    InvalidJson,
    InvalidRequest,
    JoinGame(String),
    OpenLobbies(Vec<String>), // TODO: Vec<GameMetaData>
    RunningGames(Vec<String>), // TODO: Vec<GameMetaData>
}

struct Message {
    sender: Arc<Mutex<APIClient>>, // QUES: what exactly does 'static do here?
    content: Vec<u8>,
}

impl Message {
    fn new(sender: Arc<Mutex<APIClient>>, content: Vec<u8>) -> Message { // QUES: what exactly does 'static do here?
        log::info!("new Message was created"); // QUES: better identifier
        Message{sender, content}
    }
}




// JAVASCRIPT TEST

// let socket = new WebSocket('ws://localhost:8001');
// let m = '"JoinDivGameNormal"';
// let tmp = undefined;
// let tmp2 = undefined;
// socket.onopen = function(event) {
// 	socket.send(m);

// 	socket.onmessage = function(event) {
// 		tmp = event;
// 		tmp.data.text().then(res => {
// 			tmp2 = res; console.log(res);
// 			console.log(event);
//         });
//     }

// 	socket.onclose = function(event) {
// 		console.log("connection closed");
// 		console.log(event);
//     }
// }