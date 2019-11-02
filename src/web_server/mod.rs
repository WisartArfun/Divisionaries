//! creates a simple web server
//!
//! The web server uses actix-web and the user can pass a configuration to the web server.

mod server;

pub use server::{ProvideService, WebServer};

pub mod utils {
    //! utils for working with files and http responses

    use log;

    use actix_web::{http, HttpResponse};

    use crate::file_manager;

    /// takes a `Vec<u8>` and a mime type and turns them into an `HttpResponse`
    /// 
    /// # Arguments
    /// 
    /// * content: `Vec<u8>` - the content of the `HttpResponse`
    /// * mime: `&str` - the mime type of the `HttpResponse`
    /// 
    /// # Returns 
    /// 
    /// * http_response: `HttpResponse` - an http response with the content in the body and a defined mime type
    pub fn get_responder(content: Vec<u8>, mime: &str) -> HttpResponse {
        // QUES: when to use into
        HttpResponse::Ok()
            .header(http::header::CONTENT_TYPE, mime)
            .body(content)
    }

    /// reads a file and returns the content as an `HttpResponse`
    /// 
    /// # Arguments
    /// 
    /// * src: `&str` - the path to the file
    /// * mime: `&str` - the mime type of the `HttpResponse`
    /// 
    /// # Returns
    /// 
    /// * http_response: `Result<HttpResponse, std::io::Error>` - a result with a `HttpResponse`
    /// 
    /// # Errors
    /// 
    /// Returns an `std::io::Error` if `file_manager::read_file(src)` returns an error.
    /// 
    /// # Example
    /// 
    /// This example turns the content of `tests/index.html` into a `HttpResponse`, it should run without panicking.
    /// 
    /// ```rust
    /// use bucketer::web_server::utils;
    /// 
    /// let response = utils::get_file("tests/index.html", "text/html").unwrap();
    /// ```
    /// 
    /// This example will fail, as the file specified does not exist.
    /// 
    /// ```rust, should_panic
    /// use bucketer::web_server::utils;
    /// 
    /// let response = utils::get_file("tests/does_not_exist.html", "text/html").unwrap();
    /// ```
    pub fn get_file(src: &str, mime: &str) -> Result<HttpResponse, std::io::Error> {
        // IDEA: get mime automatically
        log::debug!("getting file from {}", src);
        let content = file_manager::read_file(src)?;
        Ok(get_responder(content.into_bytes(), mime))
    }

    /// reads a file and returns the content as an `HttpResponse` after replacing some identifiers
    /// 
    /// # Arguments
    /// 
    /// * src: `&str` - the path to the file
    /// * mime: `&str` - the mime type of the `HttpResponse`
    /// * replacers: `&[(&str, &str)]` - an `array` with a `tuple` containing old and new
    /// 
    /// # Returns
    /// 
    /// * http_response: `Result<HttpResponse, std::io::Error>` - a result with a `HttpResponse`
    /// 
    /// # Errors
    /// 
    /// Returns an `std::io::Error` if `file_manager::read_file(src)` returns an error.
    /// 
    /// # Example
    /// 
    /// This example turns the content of `tests/index.html` into a `HttpResponse` and replaces all occurences of `##IP##` with `127.0.0.1`.
    /// It should run without an error.
    /// 
    /// ```rust
    /// use bucketer::web_server::utils;
    /// 
    /// let response = utils::get_replace("tests/index.html", "text/html", &[("##IP##", "127.0.0.1")]).unwrap();
    /// ```
    /// 
    /// This example will fail, as the file specified does not exist.
    /// 
    /// ```rust, should_panic
    /// use bucketer::web_server::utils;
    /// 
    /// let response = utils::get_replace("tests/does_not_exist.html", "text/html", &[("##IP##", "127.0.0.1")]).unwrap();
    /// ```
    pub fn get_replace(src: &str, mime: &str, replacers: &[(&str, &str)]) -> Result<HttpResponse, std::io::Error> {
        log::debug!("getting file from {} with replacers", src);
        let mut content = file_manager::read_file(src)?;
        for (old, new) in replacers {
            content = content.replace(old, new);
        }
        Ok(get_responder(content.into_bytes(), mime))
    }
}
