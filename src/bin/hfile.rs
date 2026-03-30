use anyhow::{Context, Result, bail};
use clap::Parser;
use hfile::{checksum, command::Args, hash, walkdir};
use path_clean::PathClean;
use std::{
    fs,
    io::{self, BufWriter, Write},
    path::Path,
};

fn check_checksums(args: &Args, checksum_file: &str) -> Result<()> {
    checksum::check(Path::new(checksum_file), args.algorithm)?;
    Ok(())
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

    let hash = hash::hash_file(args.algorithm, file)?;
    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());
    if args.size {
        writeln!(
            output,
            "{}\t{}\t{}",
            hash,
            file.clean().display(),
            bytesize::to_string(metadata.len(), true)
        )?;
    } else {
        writeln!(output, "{hash}\t{}", file.clean().display())?;
    }
    output.flush()?;

    Ok(())
}

fn hash_path(args: &Args, path: &str) -> Result<()> {
    if args.duplicates {
        let duplicates = walkdir::find_duplicates(path, args.algorithm)?;
        let stdout = io::stdout();
        let mut output = BufWriter::new(stdout.lock());
        for (index, (hash, paths)) in duplicates.iter().enumerate() {
            writeln!(output, "{hash}")?;
            for path in paths {
                writeln!(output, "\t{}", path.clean().display())?;
            }

            if index + 1 < duplicates.len() {
                writeln!(output)?;
            }
        }
        output.flush()?;
    } else {
        walkdir::read(path, args.algorithm, args.size)?;
    }

    Ok(())
}

fn run() -> Result<()> {
    let args = Args::parse();

    if let Some(checksum_file) = &args.check {
        check_checksums(&args, checksum_file)
    } else {
        match &args.path {
            Some(path) => hash_path(&args, path),
            None => hash_single_file(&args),
        }
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
