use std::error::Error;

use log;

use bucketer::{file_manager, logger};

fn main() -> Result<(), Box<dyn Error>> {
    logger::init("config/log4rs.yaml");

    log::debug!("this is a test");

    println!("{}", file_manager::read_file("tests/read_file.txt")?);

    Ok(())
}
