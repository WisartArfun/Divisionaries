//! handles acces to files
//! 
//! # Example
//! 
//! A text file at `test/read_file.txt` contains: `"Example Text."`.
//! Here is shown how to read it to a string;
//! 
//! ```rust
//! use bucketer::file_manager;
//! 
//! let content = file_manager::read_file("tests/read_file.txt").unwrap();
//! 
//! assert_eq!(content, "Example Text.");
//! ```

use std::fs;
use std::error::Error;

/// reads the content of a file to a `String` and returns it in a `Result`
/// 
/// # Arguments
/// 
/// * src: `&str` - the path to the file to read
/// 
/// # Returns
/// 
/// file_content: `Result<String, std::io::Result>` - the content of the file specified by the `src` argument
/// 
/// # Errors
/// 
/// This function will throw an error in cases where `std::fs::read_to_string(some_path)` would return an `std::io::Error`.
/// This inclueds cases such as not having permission to access the src or it no file existing at the src.
pub fn read_file(src: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(src)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_success() {
        let content = read_file("tests/read_file.txt").unwrap();
        assert_eq!("Example Text.", content);
    }

    #[test]
    #[should_panic(expected = "Error opening file")]
    fn read_file_fails() {
        read_file("tests/does_not_exist.txt").expect("Error opening file");
    }
}