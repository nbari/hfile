use anyhow::Result;
use walkdir::WalkDir;

pub fn read(dir: &str) -> Result<String> {
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        println!("{}", entry.path().display());
    }
    Ok(String::from("ok"))
}
