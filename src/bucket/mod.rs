//! holds all data types needed to create a custom `Bucket`

mod server;
mod types;
mod manager;

pub use manager::BucketManager;
pub use server::{BucketData, BucketServer, ConnectionHandler}; // QUES: ConnectionHandler needed? better braodcast solution???
pub use types::{Bucket, BucketClient, BucketMessage};
