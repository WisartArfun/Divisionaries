use std::str;

use log;

use actix_web::{HttpResponse, http, Responder};

use crate::data_manager::file_manager::FileManager;


fn get_utf8<S: Into<Vec<u8>>>(content: S, mime: &str) -> HttpResponse { // PROB: impl Responder // QUES: &mut faster/better? // QUES: &s => type annotations needed, why?
    log::debug!("getting utf8 for mime {}", mime);
    HttpResponse::Ok()
    .header(http::header::CONTENT_TYPE, mime)
    .body(content.into())
}

pub fn get_file(src: &str, mime: &str) -> impl Responder {
    log::info!("get_file from {} with mime {}", src, mime);
    if let Some(file_content) = FileManager::read_file(src) {
        return get_utf8(file_content, mime);
    }

    log::warn!("file not found at {} - returning 404 not found", src);
    HttpResponse::NotFound().body("404 - NOT FOUND!!!")
}

pub fn get_file_with_replace(src: &str, mime: &str, replacers: &[(&str, &str)]) -> impl Responder { // WARN: redundant code
    log::info!("get_file from {} with mime {}", src, mime);
    if let Some(file_content) = FileManager::read_file(src) {
        let file_content = get_replace(str::from_utf8(&file_content).unwrap(), replacers); // WARN: better solution
        return get_utf8(file_content, mime);
    }

    log::warn!("file not found at {} - returning 404 not found", src);
    HttpResponse::NotFound().body("404 - NOT FOUND!!!")
}

fn get_replace<S: Into<String>>(content: S, replacers: &[(&str, &str)]) -> String {
    log::info!("get_replace");
    let mut content = content.into();
    for replacer in replacers {
        let (old, new) = replacer;
        content = content.replace(old, new);
    }

    return content;
}




///////////
// TESTS //
///////////

#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    fn test_get_replace() {
        assert_eq!(get_replace("foo", &[("foo", "bu")]), "bu");
    }
}