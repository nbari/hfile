pub mod command;
pub mod hash;
pub mod walkdir;

use std::{fs, path::Path};

pub fn get_file_size(path: &Path) -> u64 {
    fs::metadata(path).map(|m| m.len()).unwrap_or(0)
}
