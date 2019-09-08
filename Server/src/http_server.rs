pub mod ports;
pub mod game_manager;

use std::{thread, str};
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

use actix_web::{Responder, web, App, HttpServer}; // httpResponse vs Response ?
use actix_http::{http, Response};

use game_manager::GAMEMANAGER;

// UTIL
fn read_file<T: Into<String>>(src: T) -> Option<Vec<u8>> {
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
    if let Some(file_content) = read_file(src) {
        return get_utf8(file_content, mime);
    }
    Response::Ok().body("404 - NOT FOUND!!!") // return a real 404
}

fn get_replace(src: &str, replacers: &[(&str, &str)], mime: &str) -> Response { // change to impl Responder ??
    if let Some(file_content) = read_file(src) {
        let mut file_content = str::from_utf8(&file_content).unwrap().to_string();
        
        for replacer in replacers {
            let (old, new) = replacer;
            file_content = file_content.replace(old, new);
        }

        return get_utf8(file_content, mime);
    }
    Response::Ok().body("404 - NOT FOUND!!!") // return a real 404
}

// HTML
fn get_index_html() -> impl Responder {
    get_file("Client/index.html", "text/html")
}

fn get_new_game_lobby_html(lobby_id: web::Path<(String)>) -> impl Responder {
    if let Some(game) = GAMEMANAGER.lock().unwrap().get_game_instance(&lobby_id) {
        game.lock().unwrap().start();
        let ip = game.lock().unwrap().ip.clone();
        let port = game.lock().unwrap().port.clone();
        return get_replace("Client/files/game.html", &[("#ID#", &lobby_id), ("#IP#", &ip), ("#PORT#", &port)], "text/html");
    }
    
    Response::Ok().body("No available port") // panic! ???

    // get_replace("Client/files/game.html", &[("#ID#", &lobby_id)], "text/html")
}

fn get_new_game_html(game_id: web::Path<(String)>) -> impl Responder {
    get_replace("Client/files/game_template.html", &[("#ID#", &game_id)], "text/html")
}

fn get_html(file_name: web::Path<(String)>) -> impl Responder {
    get_file(&format!("Client/files/{}", &file_name), "text/html")
}

// JAVASCRIPT
fn get_game_template_js(game_id: web::Path<(String)>) -> Response { // return impl responder
    // if let Some(game) = GAMEMANAGER.lock().unwrap().get_game_instance(&game_id) {
    //     game.lock().unwrap().start();
    //     let ip = game.lock().unwrap().ip.clone();
    //     let port = game.lock().unwrap().port.clone();
    //     return get_replace("Client/scripts/game_template.js", &[("#IP#", &ip), ("#PORT#", &port)], "text/html");
    // }
    
    // Response::Ok().body("No available port") // panic! ???
    Response::Ok().body("Fail")
}

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
                    // HTML
                    .route("/", web::get().to(get_index_html))
                    .route("/index.html", web::get().to(get_index_html))
                    .service(web::resource("/files/{file_name}").to(get_html))
                    .service(web::resource("/games/{game_id}").to(get_new_game_lobby_html))
                    .service(web::resource("/game_template/{game_id}").to(get_new_game_html))
                    // JS
                    .service(web::resource("/scripts/game_template/{game_id}").to(get_game_template_js))
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