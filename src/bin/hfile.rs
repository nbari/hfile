use anyhow::{Context, Result, bail};
use clap::Parser;
use hfile::{
    command::{Algorithm, Args},
    hash, walkdir,
};
use path_clean::PathClean;
use std::{fs, path::Path};

fn hash_file(algo: Algorithm, file: &Path) -> Result<String> {
    let hash = match algo {
        Algorithm::Md5 => hash::md5(file),
        Algorithm::Sha1 => hash::sha1(file),
        Algorithm::Sha256 => hash::sha256(file),
        Algorithm::Sha384 => hash::sha384(file),
        Algorithm::Sha512 => hash::sha512(file),
        Algorithm::Blake => hash::blake3(file),
    };

    hash.with_context(|| format!("failed to hash {}", file.display()))
}

fn hash_single_file(args: &Args) -> Result<()> {
    let Some(file_arg) = &args.file else {
        return Ok(());
    };

    let file = Path::new(file_arg);
    let metadata = fs::metadata(file)
        .with_context(|| format!("failed to read metadata for {}", file.display()))?;
    if metadata.is_dir() {
        bail!("Use option -p to pass a directory");
    }

    let hash = hash_file(args.algorithm, file)?;
    if args.size {
        println!(
            "{}\t{}\t{}",
            hash,
            file.clean().display(),
            bytesize::to_string(metadata.len(), true)
        );
    } else {
        println!("{hash}\t{}", file.clean().display());
    }

    Ok(())
}

fn hash_path(args: &Args, path: &str) -> Result<()> {
    if args.duplicates {
        let duplicates = walkdir::find_duplicates(path, args.algorithm)?;
        for (index, (hash, paths)) in duplicates.iter().enumerate() {
            println!("{hash}");
            for path in paths {
                println!("\t{}", path.clean().display());
            }

            if index + 1 < duplicates.len() {
                println!();
            }
        }
    } else {
        walkdir::read(path, args.algorithm, args.size)?;
    }

    Ok(())
}

fn run() -> Result<()> {
    let args = Args::parse();

    match &args.path {
        Some(path) => hash_path(&args, path),
        None => hash_single_file(&args),
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
