use crate::get_file_size;
use crate::{command::Algorithm, hash};
use anyhow::{Result, anyhow};
use path_clean::PathClean;
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, SyncSender},
    },
    thread,
};
use walkdir::WalkDir;

struct HashResult {
    hash: String,
    path: PathBuf,
    file_size: Option<u64>,
}

enum WorkerResult {
    Hashed(HashResult),
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

fn next_path(receiver: &Mutex<Receiver<PathBuf>>) -> Result<Option<PathBuf>> {
    let message = receiver
        .lock()
        .map_err(|error| anyhow!(error.to_string()))?
        .recv();

    match message {
        Ok(path) => Ok(Some(path)),
        Err(_) => Ok(None),
    }
}

fn spawn_workers(
    algo: Algorithm,
    include_size: bool,
    task_receiver: &Arc<Mutex<Receiver<PathBuf>>>,
    result_sender: &mpsc::Sender<WorkerResult>,
) -> Vec<thread::JoinHandle<()>> {
    (0..worker_threads())
        .map(|_| {
            let task_receiver = Arc::clone(task_receiver);
            let result_sender = result_sender.clone();
            thread::spawn(move || {
                loop {
                    let Some(path) = (match next_path(&task_receiver) {
                        Ok(path) => path,
                        Err(error) => {
                            let _ = result_sender.send(WorkerResult::Error(error));
                            return;
                        }
                    }) else {
                        return;
                    };

                    let result = hash::hash_file(algo, &path).map(|hash| HashResult {
                        hash,
                        file_size: include_size.then(|| get_file_size(&path)),
                        path,
                    });

                    let message = match result {
                        Ok(result) => WorkerResult::Hashed(result),
                        Err(error) => WorkerResult::Error(error),
                    };

                    if result_sender.send(message).is_err() {
                        return;
                    }
                }
            })
        })
        .collect()
}

fn queue_paths(dir: &str, sender: &SyncSender<PathBuf>) -> Result<usize> {
    let mut queued = 0;

    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            sender
                .send(path.to_owned())
                .map_err(|_| anyhow!("worker queue closed unexpectedly"))?;
            queued += 1;
        }
    }

    Ok(queued)
}

fn print_hash_result(result: &HashResult) {
    if let Some(file_size) = result.file_size {
        println!(
            "{}\t{}\t{}",
            result.hash,
            result.path.clean().display(),
            bytesize::to_string(file_size, true)
        );
    } else {
        println!("{}\t{}", result.hash, result.path.clean().display());
    }
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
    let (task_sender, task_receiver) = mpsc::sync_channel(queue_capacity(threads));
    let task_receiver = Arc::new(Mutex::new(task_receiver));
    let (result_sender, result_receiver) = mpsc::channel();
    let handles = spawn_workers(algo, size, &task_receiver, &result_sender);
    let queued = queue_paths(dir, &task_sender)?;
    drop(task_sender);

    for _ in 0..queued {
        match result_receiver
            .recv()
            .map_err(|_| anyhow!("worker result channel closed unexpectedly"))?
        {
            WorkerResult::Hashed(result) => print_hash_result(&result),
            WorkerResult::Error(error) => eprintln!("{error}"),
        }
    }

    join_workers(handles)
}

/// # Errors
/// Returns an error if the directory cannot be traversed or a worker fails.
pub fn find_duplicates(dir: &str, algo: Algorithm) -> Result<BTreeMap<String, Vec<PathBuf>>> {
    let threads = worker_threads();
    let (task_sender, task_receiver) = mpsc::sync_channel(queue_capacity(threads));
    let task_receiver = Arc::new(Mutex::new(task_receiver));
    let (result_sender, result_receiver) = mpsc::channel();
    let handles = spawn_workers(algo, false, &task_receiver, &result_sender);
    let queued = queue_paths(dir, &task_sender)?;
    drop(task_sender);

    let mut seen = BTreeMap::<String, PathBuf>::new();
    let mut duplicates = BTreeMap::<String, Vec<PathBuf>>::new();

    for _ in 0..queued {
        match result_receiver
            .recv()
            .map_err(|_| anyhow!("worker result channel closed unexpectedly"))?
        {
            WorkerResult::Hashed(result) => {
                if let Some(existing_path) = seen.get(&result.hash) {
                    let paths = duplicates
                        .entry(result.hash.clone())
                        .or_insert_with(|| vec![existing_path.clone()]);
                    paths.push(result.path);
                } else {
                    seen.insert(result.hash, result.path);
                }
            }
            WorkerResult::Error(error) => eprintln!("{error}"),
        }
    }

    join_workers(handles)?;

    for paths in duplicates.values_mut() {
        paths.sort();
    }

    Ok(duplicates)
}
