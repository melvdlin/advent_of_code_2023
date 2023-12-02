use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub type DynResult<T> = Result<T, Box<dyn Error>>;

pub fn load_input(path: impl AsRef<Path>) -> std::io::Result<String> {
    let mut result = String::new();
    File::open(path)?.read_to_string(&mut result)?;
    Ok(result)
}
