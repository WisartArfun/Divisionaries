use crate::logic::bucket_server::BaseBucketMessage;

pub trait Bucket: Send {
    fn start(&mut self);

    fn stop(&mut self);

    fn handle_message(&mut self, message: BaseBucketMessage);
}