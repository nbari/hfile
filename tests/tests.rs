use anyhow::{Result, anyhow};
use hfile::{command::Algorithm, hash, walkdir};
use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new(name: &str) -> Result<Self> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| anyhow!(error.to_string()))?
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("hfile-{name}-{timestamp}-{}", std::process::id()));
        fs::create_dir_all(&path)?;
        Ok(Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[test]
fn hash_md5() {
    let result = hash::md5(Path::new("tests/test-file"));
    assert!(matches!(
        result.as_deref(),
        Ok("37c4b87edffc5d198ff5a185cee7ee09")
    ));
}

#[test]
fn hash_sha1() {
    let result = hash::sha1(Path::new("tests/test-file"));
    assert!(matches!(
        result.as_deref(),
        Ok("be417768b5c3c5c1d9bcb2e7c119196dd76b5570")
    ));
}

#[test]
fn hash_sha256() {
    let result = hash::sha256(Path::new("tests/test-file"));
    assert!(matches!(
        result.as_deref(),
        Ok("c03905fcdab297513a620ec81ed46ca44ddb62d41cbbd83eb4a5a3592be26a69")
    ));
}

#[test]
fn hash_sha384() {
    let result = hash::sha384(Path::new("tests/test-file"));
    assert!(matches!(
        result.as_deref(),
        Ok(
            "f565ad8f9c76cf8c4a2e145e712df740702e066a5908f6285eafa1a83a623e882207643ce5ec29628ff0186150275ef3"
        )
    ));
}

#[test]
fn hash_sha512() {
    let result = hash::sha512(Path::new("tests/test-file"));
    assert!(matches!(
        result.as_deref(),
        Ok(
            "a12ac6bdd854ac30c5cc5b576e1ee2c060c0d8c2bec8797423d7119aa2b962f7f30ce2e39879cbff0109c8f0a3fd9389a369daae45df7d7b286d7d98272dc5b1"
        )
    ));
}

#[test]
fn hash_blake() {
    let result = hash::blake3(Path::new("tests/test-file"));
    assert!(matches!(
        result.as_deref(),
        Ok("9a689455c65ca329fbcae5a1ae8725d88c7a6fbc82fd25bbcd9370ad9c272c50")
    ));
}

#[test]
fn find_duplicates_keeps_paths_with_spaces() -> Result<()> {
    let test_dir = TestDir::new("duplicates")?;
    fs::write(test_dir.path().join("dup 1.txt"), "duplicate content")?;
    fs::write(test_dir.path().join("dup 2.txt"), "duplicate content")?;
    fs::write(test_dir.path().join("unique.txt"), "different content")?;

    let path = test_dir
        .path()
        .to_str()
        .ok_or_else(|| anyhow!("temporary directory path is not valid UTF-8"))?;
    let duplicates = walkdir::find_duplicates(path, Algorithm::Blake)?;

    assert_eq!(duplicates.len(), 1);

    let duplicate_paths = duplicates
        .values()
        .next()
        .ok_or_else(|| anyhow!("missing duplicate entry"))?;

    assert_eq!(duplicate_paths.len(), 2);
    assert!(
        duplicate_paths
            .iter()
            .any(|path| path.ends_with("dup 1.txt"))
    );
    assert!(
        duplicate_paths
            .iter()
            .any(|path| path.ends_with("dup 2.txt"))
    );

    Ok(())
}
