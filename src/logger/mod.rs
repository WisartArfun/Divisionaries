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
/// 
/// Here is an example config file,
/// more details can be found in the documentation for `log4rs`.
/// 
/// ```yaml
/// refresh_rate: 30 seconds
/// appenders:
// /  stdout_detailed:
///     kind: console
///     encoder:
///       pattern: "{f}{n}{M} - l: {L}{n}{d(%Y-%m-%d %H:%M:%S %Z)(utc)} [{h({l})}] - {m}{n}"
///   stdout:
///     kind: console
///     encoder:
///       pattern: "{d(%H:%M:%S %Z)(utc)} [{h({l})}] - {m}{n}"
///   file:
///     kind: file
///     path: "logs/log.log"
///     encoder:
///       pattern: "{f}{n}{M} - l: {L}{n}{d(%Y-%m-%d %H:%M:%S %Z)(utc)} [{h({l})}] - {m}{n}"
/// root:
///   level: debug
///   appenders:
///     - stdout
///     - file
/// loggers:
///   app::backend::db:
///     level: debug
///   app::requests:
///     level: debug
///     appenders:
///       - file
///     additive: false
/// ```
pub fn init(src: &str) {
    log4rs::init_file(src, Default::default()).unwrap(); // this is global
    log::info!("log4rs initialized from config file at src: {}", src);
}
