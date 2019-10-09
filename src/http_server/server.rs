// use std::thread;

// use actix_web::{App, HttpServer};

// use log;

// use super::trait_provide_service::ProvideService;
// use super::game_service_provider::GameServiceProvider;

// pub struct HttpGameServer {
//     ip: String,
//     port: String,
//     running: bool,
// }

// impl HttpGameServer {
//     pub fn new<S: Into<String>>(ip: S, port: S) -> HttpGameServer {
//         let ip = ip.into();
//         let port = port.into();
//         log::info!("creating new HttpGameServer with address: {}:{}", &ip, &port);
//         HttpGameServer{
//             ip,
//             port,
//             running: false,
//         }
//     }

//     pub fn start(&mut self) -> thread::JoinHandle<std::io::Result<()>> {
//         if self.running {return;}
//         self.running = true;

//         let ip = self.ip.clone();
//         let port = self.port.clone();
//         log::info!("starting HttpGameServer on address: {}:{}", &ip, &port);

//         thread::spawn(move || -> std::io::Result<()> {
//             log::debug!("started new thread for HttpGameServer");
//             HttpServer::new(|| {
//                 App::new()
//                     .configure(GameServiceProvider::configure_services)
//             })
//             .bind(format!("{}:{}", &ip, &port))
//             .unwrap()
//             .run()
//             .unwrap();

//             Ok(())
//         })
//     }
// }