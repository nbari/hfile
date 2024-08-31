use crate::get_file_size;
use crate::{
    command::Algorithm,
    hash::{blake3, md5, sha1, sha256, sha384, sha512},
};
use anyhow::{anyhow, Result};
use futures::stream::{FuturesUnordered, StreamExt};
use path_clean::PathClean;
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::task;
use walkdir::WalkDir;

/// # Errors
/// if checksum fails
pub async fn read(dir: &str, algo: Algorithm, size: bool) -> Result<()> {
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
            tasks.push(task::spawn(async move {
                match checksum(algo, path).await {
                    Ok((s, p)) => {
                        if size {
                            let file_size = get_file_size(&p);
                            println!(
                                "{}\t{}\t{}",
                                s,
                                p.clean().display(),
                                bytesize::to_string(file_size, true)
                            );
                        } else {
                            println!("{}\t{}", s, p.clean().display());
                        }
                    }
                    Err(e) => eprintln!("{e}"),
                }
            }));

            if tasks.len() == threads {
                if let Some(r) = tasks.next().await {
                    match r {
                        Ok(()) => {}
                        Err(e) => return Err(anyhow!("{}", e)),
                    }
                }
            }
        }
    }

    // consume remaining tasks
    while let Some(r) = tasks.next().await {
        match r {
            Ok(()) => {
                // Task completed successfully
            }
            Err(e) => return Err(anyhow!("{}", e)),
        }
    }

    Ok(())
}

/// # Panics
/// Panics if the lock is poisoned
/// # Errors
/// Returns an error if the lock is poisoned
pub async fn find_duplicates(
    dir: &str,
    algo: Algorithm,
) -> Result<Arc<Mutex<BTreeMap<String, PathBuf>>>> {
    let hash_map: Arc<Mutex<BTreeMap<String, PathBuf>>> = Arc::new(Mutex::new(BTreeMap::new()));
    let dup_map: Arc<Mutex<BTreeMap<String, PathBuf>>> = Arc::new(Mutex::new(BTreeMap::new()));

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
            let hash_map = Arc::clone(&hash_map);
            let dup_map = Arc::clone(&dup_map);
            tasks.push(task::spawn(async move {
                if let Ok((s, p)) = checksum(algo, path).await {
                    let mut map = hash_map.lock().expect("Failed to lock hash_map");
                    let mut dmap = dup_map.lock().expect("Failed to lock dup_map");
                    let hash = s.clone();
                    let path = p.clone();
                    map.entry(s)
                        .and_modify(|file| {
                            let dups =
                                format!("{} {}", file.clean().display(), path.clean().display());
                            dmap.entry(hash)
                                .and_modify(|value| {
                                    let dups = format!(
                                        "{} {}",
                                        value.clean().display(),
                                        path.clean().display()
                                    );
                                    *value = PathBuf::from(dups);
                                })
                                .or_insert_with(|| dups.into());
                        })
                        .or_insert(p);
                }
            }));

            if tasks.len() == threads {
                if let Some(r) = tasks.next().await {
                    match r {
                        Ok(()) => {}
                        Err(e) => return Err(anyhow!("{}", e)),
                    }
                }
            }
        }
    }

    // consume remaining tasks
    while let Some(r) = tasks.next().await {
        match r {
            Ok(()) => {
                // Task completed successfully
            }
            Err(e) => return Err(anyhow!("{}", e)),
        }
    }

    Ok(dup_map)
}

async fn checksum(algo: Algorithm, path: PathBuf) -> Result<(String, PathBuf)> {
    let hash = match algo {
        Algorithm::Md5 => md5(&path)?,
        Algorithm::Sha1 => sha1(&path)?,
        Algorithm::Sha256 => sha256(&path)?,
        Algorithm::Sha384 => sha384(&path)?,
        Algorithm::Sha512 => sha512(&path)?,
        Algorithm::Blake => blake3(&path)?,
    };
    Ok((hash, path))
}
