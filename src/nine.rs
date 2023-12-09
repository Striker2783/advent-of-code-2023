use std::{error::Error, fs, path::Path};

pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(())
}
