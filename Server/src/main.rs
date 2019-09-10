mod http_server;
mod logic;
mod web_socket;

fn main() -> std::io::Result<()> {
    let mut http_game_server = http_server::GameHttpServer::new("127.0.0.1", "8000");
    http_game_server.start();

    if let Some(handle) = http_game_server.handle {
        let _ = handle.join().unwrap();
    }

    Ok(())
}