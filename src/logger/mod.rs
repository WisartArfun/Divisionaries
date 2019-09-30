//! Configures and manages the logging.

use log;
use log4rs;

/// Initializes the root logger from the rust log crate using a config file.
/// 
/// The task of the SimpleLogger is to manage the logging.
/// This is done with a config file. The SimpleLogger uses the `log` crate,
/// which is a lightweight logging facade, and `log4rs` crate,
/// which is inspired by the java log4j library.
pub struct SimpleLogger;

impl SimpleLogger {
    /// Initializes the root logger using a config file. # root logger the right word???
    /// 
    /// Takes a the path to a config file, which is used to configer the logging using the `log4rs` crate.
    /// 
    /// # Arguments
    /// 
    /// * `src` - a string slice that holds the path to a config file
    /// 
    /// # Examples
    /// 
    /// Importing, initializing and logging:
    /// ```rust, no_run
    /// use log;
    /// use bucketer::logger::SimpleLogger;
    /// 
    /// let simple_logger = SimpleLogger::init("config/log4rs.yaml");
    /// log::debug!("This is a log message on the debug level.");
    /// ```
    pub fn init(src: &str) {
        log4rs::init_file(src, Default::default()).unwrap(); // this is global
        log::info!("log4rs initialized from config file at src: {}", src);
    }
}