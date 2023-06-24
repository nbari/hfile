use crate::{
    command::Algorithm,
    hash::{blake3, md5, sha1, sha256, sha384, sha512},
};
use anyhow::{anyhow, Result};
use futures::stream::{FuturesUnordered, StreamExt};
use path_clean::PathClean;
use std::path::PathBuf;
use walkdir::WalkDir;

pub async fn read(dir: &str, algo: Algorithm) -> Result<()> {
    let mut tasks = FuturesUnordered::new();

    let threads = if num_cpus::get() - 1 == 0 {
        1
    } else {
        num_cpus::get() - 1
    };

    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let path = entry.path().to_owned();
        if path.is_file() {
            tasks.push(checksum(algo, path));
            if tasks.len() == threads {
                if let Some(r) = tasks.next().await {
                    match r {
                        Ok(_) => {}
                        Err(e) => return Err(anyhow!("{}", e)),
                    }
                }
            }
        }
    }

    // consume remaining tasks
    while let Some(r) = tasks.next().await {
        match r {
            Ok(_) => {}
            Err(e) => return Err(anyhow!("{}", e)),
        }
    }

    Ok(())
}

async fn checksum(algo: Algorithm, path: PathBuf) -> Result<()> {
    let hash = match algo {
        Algorithm::Md5 => md5(&path)?,
        Algorithm::Sha1 => sha1(&path)?,
        Algorithm::Sha256 => sha256(&path)?,
        Algorithm::Sha384 => sha384(&path)?,
        Algorithm::Sha512 => sha512(&path)?,
        Algorithm::Blake => blake3(&path)?,
    };
    println!("{}\t{}", hash, path.clean().display());
    Ok(())
}
