use std::fs::File;
use std::io::prelude::*;

pub fn write_and_empty(path: &str, content: &String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
