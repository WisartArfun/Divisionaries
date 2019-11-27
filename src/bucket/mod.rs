//! holds all data types needed to create a custom `Bucket`

mod types;
mod server;

pub use server::BucketServer;
pub use types::{Bucket, BucketClient, BucketMessage};
