use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};

use log;

use bucketer::web_server::{ProvideService, WebServer, utils};

use crate::div::Config;

fn load_config() -> Config {
    Config::new("config/Settings.toml").unwrap_or_else(|err| {
        log::error!("an error occured while reading the config file: {:?}", err);
        panic!("error while reading config file:\n\t: {:?}", err); // QUES: use default config?
    })
}

pub struct ServiceProvider;

impl ProvideService for ServiceProvider {
    fn configure_services(cfg: &mut web::ServiceConfig) {
        log::debug!("configuring ServiceProvider");
        cfg.service(web::resource("/").to(ServiceProvider::html_index))
        .service(web::resource("/scripts/{name}").to(ServiceProvider::script));
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
        let config = load_config();
        utils::get_replace("client/files/index.html", "text/html", &[("##API-IP##", &config.api_ip), ("##API-PORT##", &config.api_port)]).unwrap_or_else(|_err| {
            Self::html_not_found()
        })
    }

    fn script(name: web::Path<(String)>) -> impl Responder {
        log::debug!("getting script from ServiceProvider with name: {}", &name);
        utils::get_file(&format!("client/scripts/{}",  name), "application/javascript")
    }
}
