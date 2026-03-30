use crate::get_file_size;
use crate::{command::Algorithm, hash};
use anyhow::{Result, anyhow};
use crossbeam_channel::{Receiver, Sender, bounded};
use path_clean::PathClean;
use std::{
    collections::{BTreeMap, HashMap},
    io::{self, BufWriter, Write},
    path::PathBuf,
    thread,
};
use walkdir::WalkDir;

struct ReadHashResult {
    hash: String,
    path: PathBuf,
    file_size: Option<u64>,
}

struct DuplicateHashResult {
    hash: Vec<u8>,
    path: PathBuf,
}

enum ReadWorkerResult {
    Hashed(ReadHashResult),
    Error(anyhow::Error),
}

enum DuplicateWorkerResult {
    Hashed(DuplicateHashResult),
    Error(anyhow::Error),
}

#[must_use]
fn worker_threads() -> usize {
    thread::available_parallelism().map_or(1, |count| count.get().saturating_sub(1).max(1))
}

#[must_use]
fn queue_capacity(threads: usize) -> usize {
    threads.saturating_mul(2).max(1)
}

fn spawn_read_workers(
    algo: Algorithm,
    include_size: bool,
    task_receiver: &Receiver<PathBuf>,
    result_sender: &Sender<ReadWorkerResult>,
) -> Vec<thread::JoinHandle<()>> {
    (0..worker_threads())
        .map(|_| {
            let task_receiver = task_receiver.clone();
            let result_sender = result_sender.clone();
            thread::spawn(move || {
                while let Ok(path) = task_receiver.recv() {
                    let result = hash::hash_file_for_walk(algo, &path).map(|hash| ReadHashResult {
                        hash,
                        file_size: include_size.then(|| get_file_size(&path)),
                        path,
                    });

                    let message = match result {
                        Ok(result) => ReadWorkerResult::Hashed(result),
                        Err(error) => ReadWorkerResult::Error(error),
                    };

                    if result_sender.send(message).is_err() {
                        return;
                    }
                }
            })
        })
        .collect()
}

fn spawn_duplicate_workers(
    algo: Algorithm,
    task_receiver: &Receiver<PathBuf>,
    result_sender: &Sender<DuplicateWorkerResult>,
) -> Vec<thread::JoinHandle<()>> {
    (0..worker_threads())
        .map(|_| {
            let task_receiver = task_receiver.clone();
            let result_sender = result_sender.clone();
            thread::spawn(move || {
                while let Ok(path) = task_receiver.recv() {
                    let result = hash::hash_file_bytes_for_walk(algo, &path)
                        .map(|hash| DuplicateHashResult { hash, path });

                    let message = match result {
                        Ok(result) => DuplicateWorkerResult::Hashed(result),
                        Err(error) => DuplicateWorkerResult::Error(error),
                    };

                    if result_sender.send(message).is_err() {
                        return;
                    }
                }
            })
        })
        .collect()
}

fn queue_paths(dir: &str, sender: &Sender<PathBuf>) -> Result<usize> {
    let mut queued = 0;

    for entry in WalkDir::new(dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            sender
                .send(entry.path().to_owned())
                .map_err(|_| anyhow!("worker queue closed unexpectedly"))?;
            queued += 1;
        }
    }

    Ok(queued)
}

fn spawn_path_queue(dir: String, sender: Sender<PathBuf>) -> thread::JoinHandle<Result<usize>> {
    thread::spawn(move || {
        let result = queue_paths(&dir, &sender);
        drop(sender);
        result
    })
}

fn print_hash_result(writer: &mut impl Write, result: &ReadHashResult) -> Result<()> {
    if let Some(file_size) = result.file_size {
        writeln!(
            writer,
            "{}\t{}\t{}",
            result.hash,
            result.path.clean().display(),
            bytesize::to_string(file_size, true)
        )?;
    } else {
        writeln!(writer, "{}\t{}", result.hash, result.path.clean().display())?;
    }

    Ok(())
}

fn join_workers(handles: Vec<thread::JoinHandle<()>>) -> Result<()> {
    for handle in handles {
        handle
            .join()
            .map_err(|_| anyhow!("worker thread panicked"))?;
    }

    Ok(())
}

/// # Errors
/// Returns an error if the directory cannot be traversed or a worker fails.
pub fn read(dir: &str, algo: Algorithm, size: bool) -> Result<()> {
    let threads = worker_threads();
    let (task_sender, task_receiver) = bounded(queue_capacity(threads));
    let (result_sender, result_receiver) = bounded(queue_capacity(threads));
    let handles = spawn_read_workers(algo, size, &task_receiver, &result_sender);
    let queue_handle = spawn_path_queue(dir.to_owned(), task_sender);
    drop(result_sender);

    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());
    for message in result_receiver {
        match message {
            ReadWorkerResult::Hashed(result) => print_hash_result(&mut output, &result)?,
            ReadWorkerResult::Error(error) => eprintln!("{error}"),
        }
    }
    output.flush()?;

    join_workers(handles)?;
    queue_handle
        .join()
        .map_err(|_| anyhow!("path queue thread panicked"))??;

    Ok(())
}

/// # Errors
/// Returns an error if the directory cannot be traversed or a worker fails.
pub fn find_duplicates(dir: &str, algo: Algorithm) -> Result<BTreeMap<String, Vec<PathBuf>>> {
    let threads = worker_threads();
    let (task_sender, task_receiver) = bounded(queue_capacity(threads));
    let (result_sender, result_receiver) = bounded(queue_capacity(threads));
    let handles = spawn_duplicate_workers(algo, &task_receiver, &result_sender);
    let queue_handle = spawn_path_queue(dir.to_owned(), task_sender);
    drop(result_sender);

    let mut seen = HashMap::<Vec<u8>, PathBuf>::new();
    let mut duplicates = HashMap::<Vec<u8>, Vec<PathBuf>>::new();

    for message in result_receiver {
        match message {
            DuplicateWorkerResult::Hashed(result) => {
                if let Some(existing_path) = seen.get(&result.hash) {
                    let paths = duplicates
                        .entry(result.hash.clone())
                        .or_insert_with(|| vec![existing_path.clone()]);
                    paths.push(result.path);
                } else {
                    seen.insert(result.hash, result.path);
                }
            }
            DuplicateWorkerResult::Error(error) => eprintln!("{error}"),
        }
    }

    join_workers(handles)?;
    queue_handle
        .join()
        .map_err(|_| anyhow!("path queue thread panicked"))??;

    let mut duplicate_map = BTreeMap::<String, Vec<PathBuf>>::new();
    for (hash, mut paths) in duplicates {
        paths.sort();
        duplicate_map.insert(hash::write_hex_bytes(&hash), paths);
    }

    Ok(duplicate_map)
}
