use std::io::{self, Read};
use std::thread;

mod http_server;
mod web_socket;
mod logic;

use logic::game;

fn main() -> std::io::Result<()> {
    let mut http_game_server = http_server::GameHttpServer::new("127.0.0.1", "8000");
    http_game_server.start();

    let game = game::Game::new();
    let mut web_socket = web_socket::WebSocket::new("127.0.0.1", "9001");
    web_socket.start(game.clients.clone());

    let clients = game.clients.clone();
    thread::spawn(move || -> std::io::Result<()> {
        loop {
            let clients = clients.lock().unwrap();
            for client in &clients.clients {
                let input = client.try_recv();
                if let Some(message) = input {
                    println!("message: {}", message);
                }
            } 
        }

        Ok(())
    });

    loop {
        println!("loop");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let clients = game.clients.clone();
        let clients = clients.lock().unwrap();
        for client in &clients.clients {
            client.send(&input);
        }
    }

    if let Some(handle) = web_socket.handle {
        let _ = handle.join().unwrap();
    };
    if let Some(handle) = http_game_server.handle {
        let _ = handle.join().unwrap();
    }

    Ok(())
}