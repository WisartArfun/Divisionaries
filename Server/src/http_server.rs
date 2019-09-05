use std::thread;

use std::fs::File;
use std::io::Read;
use actix_web::{Responder, web, App, HttpServer}; // httpResponse vs Response ?
use actix_http::{http, Response};

use std::sync::{Arc, Mutex};

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

fn get_new_lobby(src: &str, game_id: &str, mime: &str) -> impl Responder {
    match File::open(src) { // add path management // check if vec is slower than string => seperate depending on mime
        Ok(mut file) => {
            // let path = Path::new(&file_name); // if !path.exists() { //     return String::from("Not Found!").into(); // }
            // let mut file_content = Vec::new();
            // file.read_to_end(&mut file_content).expect("Unable to read");
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();

            let buf = buf.replace("#NUM#", game_id);
            // let file_content = buf.as_mut_vec();

            Response::Ok()
            .header(http::header::CONTENT_TYPE, mime)
            // .body(file_content)
            .body(buf) // okay that string and not utf 8?
        },
        Err(err) => {
            println!("Error: {:?}", err);
            Response::Ok().body("Hello world!")
        }
    }
}

// HTML
fn get_index_html() -> impl Responder {
    get_file("Client/index.html", "text/html")
}

fn get_new_lobby_html(game_id: web::Path<(String)>) -> impl Responder {
    get_new_lobby("Client/files/lobby.html", &game_id, "text/html")
}

fn get_html(file_name: web::Path<(String)>) -> impl Responder {
    get_file(&format!("Client/files/{}", &file_name), "text/html")
}

// JAVASCRIPT
fn get_js(script_name: web::Path<(String)>) -> impl Responder {
    get_file(&format!("Client/scripts/{}", &script_name), "application/javascript")
}

// GRAPHICS
fn get_jpeg(jpeg_name: web::Path<(String)>) -> impl Responder {
    get_file(&format!("Client/graphics/{}", &jpeg_name), "image/jpeg")
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
                App::new()
                    // INDEX
                    .route("/", web::get().to(get_index_html))
                    .route("/index.html", web::get().to(get_index_html))
                    .service(web::resource("/Client/files/{file_name}").to(get_html))
                    .service(web::resource("/Client/files/lobby/{game_id}").to(get_new_lobby_html))
                    .service(web::resource("/games/{game_id}").to(get_new_lobby_html))
                    // JS
                    .service(web::resource("/Client/scripts/{script_name}").to(get_js))
                    // GRAPHICS
                    .service(web::resource("/Client/graphics/{jpeg_name}").to(get_jpeg))
                    // NOT FOUND
                    .default_service(web::route().to(|| Response::NotFound()))
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