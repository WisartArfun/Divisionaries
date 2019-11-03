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
