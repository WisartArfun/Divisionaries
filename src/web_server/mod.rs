//! creates a simple web server
//!
//! The web server uses `actix-web` and the user can pass a configuration to the web server.

mod server; // stays private, user only needs `ProvideService` and `WebServer`
pub mod utils;

pub use server::{ProvideService, WebServer};