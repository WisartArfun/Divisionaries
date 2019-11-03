use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};

use log;

use bucketer::web_server::{ProvideService, WebServer, utils};

pub struct ServiceProvider;

impl ProvideService for ServiceProvider {
    fn configure_services(cfg: &mut web::ServiceConfig) {
        log::debug!("configuring ServiceProvider");
        cfg.service(web::resource("/").to(ServiceProvider::html_index));
    }
}

impl ServiceProvider {
    fn html_not_found() -> HttpResponse {
        HttpResponse::NotFound()
            .content_type("text/plain")
            .body("Not Found")
    }

    fn html_index() -> impl Responder { // QUES: opaque type problem
        log::debug!("getting html index from ServiceProvider");
        utils::get_file("client/files/index.html", "text/html").unwrap_or_else(|_err| {
            Self::html_not_found()
        })
    }
}

// // WARN: PROB: IDEA: NEXT: get default parser
// // DESCRIPTION: configures an actix_web::App for crate::http_server::server::GameHttpServer
// pub struct GameServiceProvider;

// impl ProvideService for GameServiceProvider {
//     fn configure_services(cfg: &mut web::ServiceConfig) {
//         log::debug!("configuring cfg for a GameServiceProvider");
//         cfg
//             // HTML
//             .service(web::resource("/").to(GameServiceProvider::get_html_index))
//             .service(web::resource("/index.html").to(GameServiceProvider::get_html_index))
//             .service(web::resource("/files/{file_name}").to(GameServiceProvider::get_html))
//             .service(web::resource("/nor_div_game_lobby/{game_id}").to(GameServiceProvider::get_html_game))
//             .service(web::resource("/games/{lobby_id}").to(GameServiceProvider::get_html_game_lobby))
//             // JS
//             .service(web::resource("/scripts/{folder}/{script_name}").to(GameServiceProvider::get_js_mod))
//             .service(web::resource("/scripts/{script_name}").to(GameServiceProvider::get_js))
//             // GRAPHICS
//             .service(web::resource("/graphics/{jpeg_name}").to(GameServiceProvider::get_jpeg));
//             // NOT FOUND
//             // .default_service(web::route().to(|| Response::NotFound())); // PROB: alternative
//     }
// }

// impl GameServiceProvider {
//     // HTML
//     fn get_html_index() -> impl Responder {
//         log::debug!("get html index from GameServiceProvider");
//         http_utils::get_file("client/files/index.html", "text/html")
//     }

//     fn get_html_game_lobby(lobby_id: web::Path<(String)>) -> impl Responder { // QUES: what to do here?
//         log::debug!("get html game lobby from GameServiceProvider");
//         log::info!("loading data from config file");
//         let mut settings = config::Config::default();
//         settings.merge(config::File::with_name("config/Settings")).unwrap(); // QUES: error handling
//         let settings = settings.try_into::<HashMap<String, String>>().unwrap();

//         let api_ip = if let Some(port) = settings.get("api_ip") {port} else {"127.0.0.1"};
//         let api_port = if let Some(port) = settings.get("api_port") {port} else {"8001"};
//         http_utils::get_file_with_replace("client/files/nor_div_game_lobby.html", "text/html", &[("#ID#", &lobby_id), ("#IP#", &api_ip), ("#PORT#", &api_port)])
//         // if let Some(game) = GAMEMANAGER.lock().unwrap().get_game_lobby(&lobby_id) {
//         //     game.lock().unwrap().start();
//         //     let ip = game.lock().unwrap().get_ip();
//         //     let port = game.lock().unwrap().get_port();
//         //     return get_replace("client/files/game.html", &[("#ID#", &lobby_id), ("#IP#", &ip), ("#PORT#", &port)], "text/html");
//         // }
        
//         // Response::Ok().body("No available port") // panic! ???
//     }

//     fn get_html_game(game_id: web::Path<(String)>) -> impl Responder {
//         log::debug!("get html game from GameServiceProbvider");
//         http_utils::get_file_with_replace("client/files/nor_div_game.html", "text/html", &[("#ID#", &game_id)])
//     }

//     fn get_html(file_name: web::Path<(String)>) -> impl Responder {
//         log::debug!("get html from: {}", &file_name);
//         http_utils::get_file(&format!("client/files/{}", &file_name), "text/html")
//     }

//     // JS
//     fn get_js(script_name: web::Path<(String)>) -> impl Responder {
//         log::debug!("get js from: {}", &script_name);
//         http_utils::get_file(&format!("client/scripts/{}", &script_name), "application/javascript")
//     }

//     fn get_js_mod(data: web::Path<(String, String)>) -> impl Responder {
//         let folder_name = &data.0;
//         let script_name = &data.1;
//         log::debug!("get js from: {}/{}", folder_name, script_name);
//         http_utils::get_file(&format!("client/scripts/{}/{}", folder_name, script_name), "application/javascript")
//     }

//     // GRAPHICS
//     fn get_jpeg(jpeg_name: web::Path<(String)>) -> impl Responder {
//         log::debug!("get jpeg from: {}", &jpeg_name);
//         http_utils::get_file(&format!("client/graphics/{}", &jpeg_name), "image/jpeg")
//     }
// }