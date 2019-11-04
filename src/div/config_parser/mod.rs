//! parses a config file

use std::collections::HashMap;

use config;
use log;

pub struct Config {
    pub api_ip: String,
    pub api_port: String,
    pub http_ip: String,
    pub http_port: String,
}

impl Config {
    pub fn new(src: &str) -> Result<Self, config::ConfigError> {
        log::info!("loading data from config file at: {}", src);
        let mut settings = config::Config::default();
        settings.merge(config::File::with_name(src))?; // QUES: error handling
        let settings = settings.try_into::<HashMap<String, String>>().unwrap();

        let api_ip = (if let Some(port) = settings.get("api_ip") {port} else {"127.0.0.1"}).to_string();
        let api_port = (if let Some(port) = settings.get("api_port") {port} else {"8001"}).to_string();

        let http_ip = (if let Some(port) = settings.get("http_ip") {port} else {"127.0.0.1"}).to_string();
        let http_port = (if let Some(port) = settings.get("http_port") {port} else {"8000"}).to_string();

        Ok(Self {
            api_ip,
            api_port,
            http_ip,
            http_port,
        })
    }
}
