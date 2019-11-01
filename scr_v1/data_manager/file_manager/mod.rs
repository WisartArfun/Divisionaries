use std::fs;
use std::io::prelude::*;

use log;

use crate::data_manager::ReadWrite;

pub struct File {
    file: fs::File,
}

impl File {
    pub fn new(file: fs::File) -> File {
        log::debug!("creating new File object");
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

pub struct FileManager;

impl FileManager {
    pub fn get_file(src: &str) -> Option<File> {
        log::info!("getting file from {}", src);
        match fs::File::open(src) { // add path management
            Ok(fs_file) => {
                Some(File::new(fs_file))
            },
            Err(err) => {
                log::error!("error: {:?}", err);
                None
            }
        }
    }
    
    pub fn read_file(src: &str) ->Option<Vec<u8>> {
        log::debug!("getting file content from {}", src);
        if let Some(mut f_file) = FileManager::get_file(src) {
            return Some(f_file.read());
        }
        None
    }
}

///////////
// TESTS //
///////////

#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    fn test_read_file_fails() {
        if FileManager::read_file("does_not_exist") != None {
            panic!("read file should return None");
        }
    }

    #[test]
    fn test_read_file_works() {
        if FileManager::read_file("src/main.rs") == None {
            panic!("read file should return Something");
        }
    }
}