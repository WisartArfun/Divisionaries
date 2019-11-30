use std::thread;

use actix_web::{web, App, HttpServer};

/// A simple, configurable WebServer.
///
/// # Type Parameters
///
/// * `'a` - for ip and port
///
/// # Variables
///
/// * ip: `&'a str` - the ip of the `WebServer`
/// * port: `&'a str` - the port of the `WebServer`
/// * running: bool - whether the `WebServer` is running or not
pub struct WebServer<'a> {
    ip: &'a str,
    port: &'a str,
    running: bool,
}

impl<'a> WebServer<'a> {
    /// creates a new `WebServer` instance
    ///
    /// # Arguments
    ///
    /// * ip: `&'a str` - the ip of the `WebServer`
    /// * port: `&'a str` - the port of the `WebServer`
    pub fn new(ip: &'a str, port: &'a str) -> Self {
        log::info!("creating a new WebServer at {}:{}", ip, port);
        Self {
            ip,
            port,
            running: false,
        }
    }

    /// starts a `WebServer` instance
    ///
    /// # Type Parameters
    ///
    /// * P: `ProvideService` - provides the services for the `WebServer`
    ///
    /// # Returns
    ///
    /// * handle: `Option<thread::JoinHandle<std::io::Result<()>>>` - handle to the thread of the `WebServer`
    pub fn start<P: ProvideService>(&mut self) -> Option<thread::JoinHandle<std::io::Result<()>>> {
        if self.running {
            return None;
        }
        self.running = true;
        log::info!("starting a WebServer instance at {}:{}", self.ip, self.port);

        let ip = self.ip.to_string();
        let port = self.port.to_string();
        let handle = thread::spawn(move || -> std::io::Result<()> {
            log::debug!("started new thread for a WebServer instance");
            HttpServer::new(move || App::new().configure(P::configure_services))
                .bind(format!("{}:{}", &ip, &port))?
                .run()?;

            Ok(())
        });

        Some(handle)
    }
}

/// allows the user to define the configuration of a `WebServer`
pub trait ProvideService: Send + Sync {
    // QUES: both needed???
    /// configures services
    fn configure_services(cfg: &mut web::ServiceConfig);
}
