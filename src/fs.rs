use crate::error::Result;
use std::{ fs, path::Path };


/// Checks whether a directory exists or not
pub fn dir_exists<P>(dir: P) -> Result<bool> where P: AsRef<Path> {
    let dir = dir.as_ref();
    Ok(dir.is_dir())
}


/// Checks whether a directory exists or not
pub fn dir_is_empty<P>(dir: P) -> Result<bool> where P: AsRef<Path> {
    for entry in fs::read_dir(dir)? {
        let _entry = entry?;
        return Ok(false)
    }
    Ok(true)
}
/// Removes all files within a given directory (but not the directory itself)
pub fn clear_dir<P>(dir: P) -> Result where P: AsRef<Path> {
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        match entry.file_type()?.is_dir() {
            true => fs::remove_dir_all(entry.path())?,
            false => fs::remove_file(entry.path())?
        }
    }
    Ok(())
}


/// Reads a string from a file
pub fn read_string<P>(path: P) -> Result<String> where P: AsRef<Path> {
    let bytes = fs::read(path)?;
    String::from_utf8(bytes).map_err(|e| einval!("Non-UTF-8 bytes in string ({})", e))
}