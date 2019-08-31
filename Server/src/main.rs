use std::fs::File;
use std::io::Read;

use actix_web::{App, Responder, HttpServer, web}; // httpResponse vs Response
use actix_http::{http, Response};


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

// MAIN
fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new() // only one handler for scripts, html, graphics?
            .route("/", web::get().to(get_index))
            .route("/Client/scripts/ProtocolInterpreter.js", web::get().to(get_protocol_interpreter))
            .route("/Client/scripts/State.js", web::get().to(get_state))
            .route("/Client/scripts/Renderer.js", web::get().to(get_renderer))
            .route("/Client/scripts/GraphicMapping.js", web::get().to(get_graphic_mapping))
            .route("/Client/graphics/crown.jpg", web::get().to(get_crown))
            .route("/Client/graphics/fog.jpg", web::get().to(get_fog))
            .route("/Client/graphics/king.jpg", web::get().to(get_empty))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .unwrap();

    Ok(())
}