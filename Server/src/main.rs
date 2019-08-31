use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;
use tungstenite;

mod http_server;

fn main() -> std::io::Result<()> {
    let mut http_game_server = http_server::GameHttpServer::new("127.0.0.1", "8000");
    http_game_server.start();

    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() { // handle connectionclosed
        println!("hello there");
        spawn (move || {
            // let mut websocket = accept(stream.unwrap(), None).unwrap();
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    // websocket.write_message(msg).unwrap();
                    websocket.write_message(tungstenite::Message::Text("21020311".to_string())).unwrap(); // nicer
                }
            }
        });
    }

    Ok(())
}