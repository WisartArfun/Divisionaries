use std::str;

use bucketer::bucket::{Bucket, BucketMessage};

pub struct TestBucket {
    turn: i64,
}

impl TestBucket {
    pub fn new() -> Self {
        Self {
            turn: 0,
        }
    }
}

impl Bucket for TestBucket {
    fn start(&mut self) {
        unimplemented!();
    }

    fn stop(&mut self) {
        unimplemented!();
    }

    fn update(&mut self) {
        // self.turn += 1;
        // if self.turn % 100 == 0 {
        //     log::debug!("[REM] TestBucket turn #{}", self.turn);
        // }
    }

    fn handle_message(&mut self, message: BucketMessage) {
        log::debug!("[REM] TestBucket received a message: {:?}", str::from_utf8(&message.get_content()));
    }
}