//! Manages access to data.
//! 
//! Data can be accessed using the `ReadWrite` trait. There are several objects that implement this trait,
//! such as `file_manager::File`.
//! The module `file_manager` manages the access to the file system of the host os.

pub mod file_manager;

/// Manages reading and writing data to objects managed by the `data_manager` module.
pub trait ReadWrite {
    /// Handles reading stored data.
    /// 
    /// # Returns
    /// 
    /// * content: `Vec<u8>` - data stored in `self`
    fn read(&mut self) -> Vec<u8>; // PROB: wont work with -> O

    /// Handles storing data.
    /// 
    /// # Type Arguments
    /// 
    /// * I: `Into<Vec<u8>>` - type of content
    /// 
    /// # Arguments
    /// 
    /// * content: `I` - content to be stored
    fn write<I: Into<Vec<u8>>>(&mut self, content: I); // QUES: right way to do this with type parameter?
}