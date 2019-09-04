mod http_server;
mod web_socket;
mod logic;

use logic::game;

fn main() -> std::io::Result<()> {
    let mut http_game_server = http_server::GameHttpServer::new("127.0.0.1", "8000");
    http_game_server.start();

    let game = game::Game::new();
    let mut web_socket = web_socket::WebSocket::new("127.0.0.1", "9001");
    web_socket.start(game.clients);

    if let Some(handle) = web_socket.handle {
        let _ = handle.join().unwrap();
    };
    if let Some(handle) = http_game_server.handle {
        let _ = handle.join().unwrap();
    }

    Ok(())
}