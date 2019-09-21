use std::fs;
use std::io::prelude::*;

use log;

use crate::connection::trait_read_write::ReadWrite;

pub struct File {
    file: fs::File,
}

impl File {
    pub fn new(file: fs::File) -> File {
        log::info!("creating new File object");
        let new_file = File{file};

        new_file
    }
}

// impl <I: Into<Vec<u8>>> ReadWrite<I, Vec<u8>> for File { // PROB: how to do this?
// impl <I: Into<Vec<u8>>> ReadWrite<I> for File {
impl ReadWrite for File {
    fn read(&mut self) -> Vec<u8> {
        log::debug!("reading from file");
        let mut file_content = Vec::new(); // INFO: vec needed to read images and binaries // QUES: check if vec is slower than string => seperate depending on mime
        self.file.read_to_end(&mut file_content).expect("Unable to read"); // PROB: better than expect for logging

        file_content
    }

    fn write<I: Into<Vec<u8>>>(&mut self, content: I) { // only utf8 at the moment
        log::debug!("writing to file");
        let content = content.into();

        self.file.write_all(&content).unwrap(); // QUES: error handling? // PROB: access denied???
    }
}