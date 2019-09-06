use std::{thread, str};

use std::fs::File;
use std::io::Read;
use actix_web::{Responder, web, App, HttpServer}; // httpResponse vs Response ?
use actix_http::{http, Response};

use std::sync::{Arc, Mutex};

// UTIL
fn read_file<T: Into<String>>(src: T, mime: T) -> Option<Vec<u8>> {
    let src = src.into();
    match File::open(&src) { // add path management // check if vec is slower than string => seperate depending on mime
        Ok(mut file) => {
            // let path = Path::new(&file_name); // if !path.exists() { //     return String::from("Not Found!").into(); // }

            let mut file_content = Vec::new(); // Vec needed for img
            file.read_to_end(&mut file_content).expect("Unable to read");

            Some(file_content)
        },
        Err(err) => {
            println!("Error: {:?}\nerror src:\t{}", err, &src);

            None
        }
    }
}

fn get_utf8<S>(content: S, mime: &str) -> Response where S: Into<Vec<u8>> { // change to impl Responder ???
    Response::Ok()
    .header(http::header::CONTENT_TYPE, mime)
    .body(content.into())
}

// GET FILE AND SET MIME
fn get_file(src: &str, mime: &str) -> impl Responder {
    if let Some(file_content) = read_file(src, mime) {
        return get_utf8(file_content, mime);
    }
    Response::Ok().body("404 - NOT FOUND!!!") // return a real 404
}

fn get_set_id(src: &str, id: &str, mime: &str) -> impl Responder {
    if let Some(file_content) = read_file(src, mime) {
        let file_content = str::from_utf8(&file_content).unwrap().replace("#ID#", id);

        return get_utf8(file_content, mime);
    }
    Response::Ok().body("404 - NOT FOUND!!!") // return a real 404
}

// HTML
fn get_index_html() -> impl Responder {
    get_file("Client/index.html", "text/html")
}

fn get_new_game_lobby_html(lobby_id: web::Path<(String)>) -> impl Responder {
    get_set_id("Client/files/game.html", &lobby_id, "text/html")
}

fn get_new_game_html(game_id: web::Path<(String)>) -> impl Responder {
    get_set_id("Client/files/game_template.html", &game_id, "text/html")
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
                    .service(web::resource("/files/{file_name}").to(get_html))
                    .service(web::resource("/games/{game_id}").to(get_new_game_lobby_html))
                    .service(web::resource("/game_template/{game_id}").to(get_new_game_html))
                    // JS
                    .service(web::resource("/scripts/{script_name}").to(get_js))
                    // GRAPHICS
                    .service(web::resource("/graphics/{jpeg_name}").to(get_jpeg))
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