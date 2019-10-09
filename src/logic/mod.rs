// pub trait Game { // QUES: more stuff???
//     fn start_server(&mut self); // QUES: does this belong here??? // QUES: return handle???

//     fn start_game(&mut self); // QUES: return handle???

//     fn update(&mut self);
// }

use crate::logic::bucket_server::BaseBucketMessage;

pub trait Bucket: Send {
    fn start(&mut self);

    fn stop(&mut self);

    fn handle_message(&mut self, message: BaseBucketMessage);
}

pub mod bucket_server;
pub mod bucket_manager;