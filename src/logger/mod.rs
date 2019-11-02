//! Configures and manages the logging.

use log;
use log4rs;

/// Initializes the root logger from the rust log crate using a config file.
///
/// This is done with a config file. It uses the `log` crate,
/// which is a lightweight logging facade, and `log4rs` crate,
/// which is inspired by the java log4j library.
///
/// # Arguments
///
/// * stc: `&str` - path to a config file
///
/// # Examples
///
/// Importing, initializing and logging:
/// 
/// ```rust
/// use log;
/// use bucketer::logger;
///
/// logger::init("tests/log4rs.yaml");
/// log::debug!("This is a log message on the debug level.");
/// ```
pub fn init(src: &str) {
    log4rs::init_file(src, Default::default()).unwrap(); // this is global
    log::info!("log4rs initialized from config file at src: {}", src);
}
