use std::error::Error;

use bucketer::file_manager;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", file_manager::read_file("tests/read_file.txt")?);

    Ok(())
}