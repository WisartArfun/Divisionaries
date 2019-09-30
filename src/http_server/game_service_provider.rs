use actix_web::{web, Responder};

use log;

use super::trait_provide_service::ProvideService;
use super::http_utils;

// DESCRIPTION: configures an actix_web::App for crate::http_server::server::GameHttpServer
pub struct GameServiceProvider {}

impl ProvideService for GameServiceProvider {
    fn configure_services(cfg: &mut web::ServiceConfig) {
        log::info!("configuring cfg for a GameServiceProvider");
        cfg
            // HTML
            .service(web::resource("/").to(GameServiceProvider::get_html_index))
            .service(web::resource("/index.html").to(GameServiceProvider::get_html_index))
            .service(web::resource("/files/{file_name}").to(GameServiceProvider::get_html))
            .service(web::resource("/game_template/{lobby_id}").to(GameServiceProvider::get_html_game_lobby))
            .service(web::resource("/games/{game_id}").to(GameServiceProvider::get_html_game))
            // JS
            .service(web::resource("/scripts/{script_name}").to(GameServiceProvider::get_js))
            // GRAPHICS
            .service(web::resource("/graphics/{jpeg_name}").to(GameServiceProvider::get_jpeg));
            // NOT FOUND
            // .default_service(web::route().to(|| Response::NotFound())); // PROB: alternative
    }
}

impl GameServiceProvider {
    // HTML
    fn get_html_index() -> impl Responder {
        log::debug!("get html index from GameServiceProvider");
        http_utils::get_file("client/files/index.html", "text/html")
    }

    fn get_html_game_lobby(lobby_id: web::Path<(String)>) -> impl Responder { // QUES: what to do here?
        log::debug!("get html game lobby from GameServiceProvider");
        http_utils::get_file_with_replace("Client/files/game.html", "text/html", &[("#ID#", &lobby_id), ("#IP#", "localhost"), ("#PORT#", "8001")])
        // if let Some(game) = GAMEMANAGER.lock().unwrap().get_game_lobby(&lobby_id) {
        //     game.lock().unwrap().start();
        //     let ip = game.lock().unwrap().get_ip();
        //     let port = game.lock().unwrap().get_port();
        //     return get_replace("Client/files/game.html", &[("#ID#", &lobby_id), ("#IP#", &ip), ("#PORT#", &port)], "text/html");
        // }
        
        // Response::Ok().body("No available port") // panic! ???
    }

    fn get_html_game(game_id: web::Path<(String)>) -> impl Responder {
        log::debug!("get html game from GameServiceProbvider");
        http_utils::get_file_with_replace("Client/files/game_template.html", "text/html", &[("#ID#", &game_id)])
    }

    fn get_html(file_name: web::Path<(String)>) -> impl Responder {
        log::debug!("get html from: {}", &file_name);
        http_utils::get_file(&format!("Client/files/{}", &file_name), "text/html")
    }

    // JS
    fn get_js(script_name: web::Path<(String)>) -> impl Responder {
        log::debug!("get js from: {}", &script_name);
        http_utils::get_file(&format!("Client/scripts/{}", &script_name), "application/javascript")
    }

    // GRAPHICS
    fn get_jpeg(jpeg_name: web::Path<(String)>) -> impl Responder {
        log::debug!("get jpeg from: {}", &jpeg_name);
        http_utils::get_file(&format!("Client/graphics/{}", &jpeg_name), "image/jpeg")
    }
}