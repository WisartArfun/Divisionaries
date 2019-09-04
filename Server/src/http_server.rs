use std::thread;

use std::fs::File;
use std::io::Read;
use actix_web::{Responder, web, App, HttpServer}; // httpResponse vs Response ?
use actix_http::{http, Response};

use std::sync::Arc;
use std::sync::Mutex;


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

fn get_game_connection() -> impl Responder {
    get_file("Client/scripts/GameConnection.js", "application/javascript")
}

fn get_game() -> impl Responder {
    get_file("Client/scripts/Game.js", "application/javascript")
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

pub struct GameHttpServer {
    ip: Arc<Mutex<String>>,
    port: Arc<Mutex<String>>,
    running: bool,
    pub handle: Option<thread::JoinHandle<std::io::Result<()>>>,
}

impl GameHttpServer {
    pub fn new<S>(ip: S, port: S) -> GameHttpServer where S: Into<String> {
        GameHttpServer {ip: Arc::new(Mutex::new(ip.into())), port: Arc::new(Mutex::new(port.into())), running: false, handle: None}
    }

    pub fn start(&mut self) {
        if self.running {return;}
        self.running = true;

        let ip = self.ip.clone();
        let port = self.port.clone();

        let handle = thread::spawn(move || -> std::io::Result<()> {
            HttpServer::new(|| {
                App::new() // only one handler for scripts, html, graphics?
                    // INDEX
                    .route("/", web::get().to(get_index))
                    // JS
                    .route("/Client/scripts/ProtocolInterpreter.js", web::get().to(get_protocol_interpreter))
                    .route("/Client/scripts/State.js", web::get().to(get_state))
                    .route("/Client/scripts/Renderer.js", web::get().to(get_renderer))
                    .route("/Client/scripts/GraphicMapping.js", web::get().to(get_graphic_mapping))
                    .route("/Client/scripts/GameConnection.js", web::get().to(get_game_connection))
                    .route("/Client/scripts/Game.js", web::get().to(get_game))
                    // GRAPHICS
                    .route("/Client/graphics/crown.jpg", web::get().to(get_crown))
                    .route("/Client/graphics/fog.jpg", web::get().to(get_fog))
                    .route("/Client/graphics/empty.jpg", web::get().to(get_empty))
            })
            .bind(format!("{}:{}", ip.lock().unwrap(), port.lock().unwrap()))
            .unwrap()
            .run()
            .unwrap();

            Ok(())
        });

        self.handle = Some(handle);
    }
}