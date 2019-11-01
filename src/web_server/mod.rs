//! creates a simple web server
//!
//! The web server uses actix-web and the user can pass a configuration to the web server.

mod server;

pub use server::{WebServer, ProvideService};

mod utils {}