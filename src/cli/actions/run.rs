use super::Action;
use crate::{checksum, hash, walkdir};
use anyhow::{Context, Result, bail};
use path_clean::PathClean;
use std::{
    fs,
    io::{self, BufWriter, Write},
    path::Path,
};

pub fn execute(action: &Action) -> Result<()> {
    match action {
        Action::HashFile {
            file,
            algorithm,
            size,
        } => hash_single_file(file, *algorithm, *size),
        Action::HashPath {
            path,
            algorithm,
            size,
        } => hash_path(path, *algorithm, *size),
        Action::FindDuplicates { path, algorithm } => find_duplicates(path, *algorithm),
        Action::CheckChecksums {
            checksum_file,
            algorithm,
        } => check_checksums(checksum_file, *algorithm),
    }
}

fn check_checksums(checksum_file: &str, algorithm: crate::cli::commands::Algorithm) -> Result<()> {
    checksum::check(Path::new(checksum_file), algorithm)?;
    Ok(())
}

fn hash_single_file(
    file_arg: &str,
    algorithm: crate::cli::commands::Algorithm,
    size: bool,
) -> Result<()> {
    let file = Path::new(file_arg);
    let metadata = fs::metadata(file)
        .with_context(|| format!("failed to read metadata for {}", file.display()))?;
    if metadata.is_dir() {
        bail!("Use option -p to pass a directory");
    }

    let hash = hash::hash_file(algorithm, file)?;
    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());
    if size {
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

fn hash_path(path: &str, algorithm: crate::cli::commands::Algorithm, size: bool) -> Result<()> {
    walkdir::read(path, algorithm, size)?;
    Ok(())
}

fn find_duplicates(path: &str, algorithm: crate::cli::commands::Algorithm) -> Result<()> {
    let duplicates = walkdir::find_duplicates(path, algorithm)?;
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

    Ok(())
}
