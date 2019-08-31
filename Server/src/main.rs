use std::fs::File;
use std::io::Read;

use std::rc::Rc;
use std::cell::Cell;

use actix_web::{Responder, web, App, Error, HttpRequest, HttpResponse, HttpServer}; // httpResponse vs Response ?
use actix_http::{http, Response};
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;

// static mut CLIENT_NUM: i32 = 0; // CHANGE THIS

// GET FILE AND SET MIME
fn get_file(src: &str, mime: &str) -> impl Responder {
    match File::open(src) { // add path management // check if vec is slower than string => seperate depending on mime
        Ok(mut file) => {
            // let path = Path::new(&file_name); // if !path.exists() { //     return String::from("Not Found!").into(); // }
            let mut file_content = Vec::new();
            file.read_to_end(&mut file_content).expect("Unable to read");
            // let mut buf = String::new(); // file.read_to_string(&mut buf).unwrap();

            Response::Ok()
            .header(http::header::CONTENT_TYPE, mime)
            .body(file_content)
        },
        Err(err) => {
            println!("Error: {:?}", err);
            Response::Ok().body("Hello world!")
        }
    }
}

// HTML
fn get_index() -> impl Responder {
    get_file("Client/index.html", "text/html") // correct?
}

// JAVASCRIPT
fn get_protocol_interpreter() -> impl Responder {
    get_file("Client/scripts/ProtocolInterpreter.js", "application/javascript")
}

fn get_state() -> impl Responder {
    get_file("Client/scripts/State.js", "application/javascript")
}

fn get_renderer() -> impl Responder {
    get_file("Client/scripts/Renderer.js", "application/javascript")
}

fn get_graphic_mapping() -> impl Responder {
    get_file("Client/scripts/GraphicMapping.js", "application/javascript")
}

fn get_connection() -> impl Responder {
    get_file("Client/scripts/Connection.js", "application/javascript")
}

// GRAPHICS
fn get_crown() -> impl Responder {
    get_file("Client/graphics/crown.jpg", "image/jpeg")
}

fn get_fog() -> impl Responder {
    get_file("Client/graphics/fog.jpg", "image/jpeg")
}

fn get_empty() -> impl Responder {
    get_file("Client/graphics/empty.jpg", "image/jpeg")
}

// // SOCKET
// struct Ws { // HTTP ACTOR
//     id: i32,
//     // count: Rc<Cell<u32>>,
// }

// impl Actor for Ws {
//     type Context = ws::WebsocketContext<Self>;
// }

// /// Handler for ws::Message message
// impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {
//     fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
//         match msg {
//             ws::Message::Ping(msg) => ctx.pong(&msg),
//             ws::Message::Text(text) => {
//                 println!("message from Client #{}: {}", self.id, text);
//                 ctx.text(text)
//             },
//             ws::Message::Binary(bin) => ctx.binary(bin),
//             _ => (),
//         }
//     }
// }


// fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
//     let resp;
//     let mut ws;
//     unsafe { // GET RID OF THIS
//         ws = Ws {id: CLIENT_NUM};
//         let ws2 = Ws{id: CLIENT_NUM};
//         resp = ws::start(ws2, &req, stream);
//         // CLIENT_NUM += 1;
//     }
//     // ws.text("hi");
//     println!("{:?}", ws);
//     resp
// }

// MAIN
fn main() -> std::io::Result<()> {
    spawn(move || -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new() // only one handler for scripts, html, graphics?
                // INDEX
                .route("/", web::get().to(get_index))
                // JS
                .route("/Client/scripts/ProtocolInterpreter.js", web::get().to(get_protocol_interpreter))
                .route("/Client/scripts/State.js", web::get().to(get_state))
                .route("/Client/scripts/Renderer.js", web::get().to(get_renderer))
                .route("/Client/scripts/GraphicMapping.js", web::get().to(get_graphic_mapping))
                .route("/Client/scripts/Connection.js", web::get().to(get_connection))
                // GRAPHICS
                .route("/Client/graphics/crown.jpg", web::get().to(get_crown))
                .route("/Client/graphics/fog.jpg", web::get().to(get_fog))
                .route("/Client/graphics/king.jpg", web::get().to(get_empty))
                // SOCKET
                // .route("/ws", web::get().to(ws_index))
        })
        .bind("127.0.0.1:8000")
        .unwrap()
        .run()
        .unwrap();

        Ok(())
    });

    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        println!("hello there");
        spawn (move || {
            // let mut websocket = accept(stream.unwrap(), None).unwrap();
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }

    Ok(())
}