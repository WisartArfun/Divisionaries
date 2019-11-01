pub mod game_service_provider;
pub mod http_utils;

use std::thread;
use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use log;

pub trait ProvideService: Send + Sync {
    fn configure_services(cfg: &mut web::ServiceConfig);
}

pub struct HttpGameServer {
    ip: String,
    port: String,
    running: bool,
}

impl HttpGameServer {
    pub fn new<S: Into<String>>(ip: S, port: S) -> HttpGameServer {
        let ip = ip.into();
        let port = port.into();
        log::info!("creating new HttpGameServer with address: {}:{}", &ip, &port);
        HttpGameServer{
            ip,
            port,
            running: false,
        }
    }

    pub fn start<P: ProvideService>(&mut self) -> thread::JoinHandle<std::io::Result<()>> {
        // if self.running {return Ok(());} // PROB: how?
        self.running = true;

        let ip = self.ip.clone();
        let port = self.port.clone();
        log::info!("starting HttpGameServer on address: {}:{}", &ip, &port);

        thread::spawn(move || -> std::io::Result<()> {
            log::debug!("started new thread for HttpGameServer");
            HttpServer::new(move || {
                App::new()
                    .configure(P::configure_services)
            })
            .bind(format!("{}:{}", &ip, &port))
            .unwrap()
            .run()
            .unwrap();

            Ok(())
        })
    }
}