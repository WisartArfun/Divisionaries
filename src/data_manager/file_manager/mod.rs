use std::fs;
use std::path;
use log;

pub mod file;

use crate::connection::trait_read_write::ReadWrite;

pub struct FileManager {}

impl FileManager {
    pub fn get_file(src: &str) -> Option<file::File> {
        log::info!("getting file from {}", src);
        match fs::File::open(src) { // add path management
            Ok(fs_file) => {
                Some(file::File::new(fs_file))
            },
            Err(err) => {
                log::error!("error: {:?}", err);
                None
            }
        }
    }
    
    pub fn read_file(src: &str) ->Option<Vec<u8>> {
        log::info!("getting file content from {}", src);
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