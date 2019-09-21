use log;
use log4rs;

pub struct SimpleLogger {}

impl SimpleLogger {
    pub fn init() {
        let src = "config/log4rs.yaml";
        log4rs::init_file(src, Default::default()).unwrap(); // this is global
        log::info!("log4rs initialized from config file at src: {}", src);
    }
}