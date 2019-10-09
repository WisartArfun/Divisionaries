pub mod bucket_server;
pub mod bucket_manager;

use crate::logic::bucket_server::{BaseBucketMessage, BaseBucketData};

pub trait Bucket: Send {
    fn start(&mut self);

    fn stop(&mut self);

    fn handle_message(&mut self, message: BaseBucketMessage);

    fn get_bucket_data(&mut self) -> BaseBucketData;
}