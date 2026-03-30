use crate::{command::Algorithm, hash};
use anyhow::{Context, Result, anyhow, bail};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

struct ChecksumEntry {
    digest: String,
    path: PathBuf,
}

#[derive(Debug, Default)]
pub struct CheckSummary {
    pub checked: usize,
    pub failed: usize,
    pub malformed: usize,
}

fn malformed_line() -> anyhow::Error {
    anyhow!("malformed checksum line")
}

fn unescape_filename(path: &str) -> Result<String> {
    let mut decoded = String::with_capacity(path.len());
    let mut chars = path.chars();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            decoded.push(ch);
            continue;
        }

        let escaped = chars.next().ok_or_else(malformed_line)?;
        match escaped {
            '\\' => decoded.push('\\'),
            'n' => decoded.push('\n'),
            'r' => decoded.push('\r'),
            't' => decoded.push('\t'),
            _ => bail!("unsupported escape sequence: \\{escaped}"),
        }
    }

    Ok(decoded)
}

fn parse_checksum_line(line: &str, algo: Algorithm) -> Result<Option<ChecksumEntry>> {
    if line.trim().is_empty() {
        return Ok(None);
    }

    let (escaped_path, body) = if let Some(rest) = line.strip_prefix('\\') {
        (true, rest)
    } else {
        (false, line)
    };

    let digest = body
        .get(..algo.digest_len())
        .ok_or_else(malformed_line)?
        .to_ascii_lowercase();
    if !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        bail!("invalid digest");
    }

    let rest = body.get(algo.digest_len()..).ok_or_else(malformed_line)?;
    let path = if let Some(path) = rest.strip_prefix('\t') {
        path
    } else if let Some(path) = rest.strip_prefix(" *") {
        path
    } else if let Some(path) = rest.strip_prefix("  ") {
        path
    } else {
        bail!("missing checksum separator");
    };

    if path.is_empty() {
        bail!("missing file path");
    }

    let path = if escaped_path {
        unescape_filename(path)?
    } else {
        path.to_owned()
    };

    Ok(Some(ChecksumEntry {
        digest,
        path: PathBuf::from(path),
    }))
}

/// # Errors
/// Returns an error if the checksum file cannot be read or any checksum fails.
pub fn check(checksum_file: &Path, algo: Algorithm) -> Result<CheckSummary> {
    let file = File::open(checksum_file)
        .with_context(|| format!("failed to open {}", checksum_file.display()))?;
    let reader = BufReader::new(file);
    let mut summary = CheckSummary::default();

    for (index, line) in reader.lines().enumerate() {
        let line_number = index + 1;
        let line = line.with_context(|| {
            format!(
                "failed to read {} at line {}",
                checksum_file.display(),
                line_number
            )
        })?;

        let entry = match parse_checksum_line(&line, algo) {
            Ok(Some(entry)) => entry,
            Ok(None) => continue,
            Err(error) => {
                summary.malformed += 1;
                eprintln!("{}:{}: {error}", checksum_file.display(), line_number);
                continue;
            }
        };

        summary.checked += 1;
        match hash::hash_file(algo, &entry.path) {
            Ok(actual) if actual.eq_ignore_ascii_case(&entry.digest) => {
                println!("{}: OK", entry.path.display());
            }
            Ok(_) => {
                summary.failed += 1;
                println!("{}: FAILED", entry.path.display());
            }
            Err(error) => {
                summary.failed += 1;
                println!("{}: FAILED", entry.path.display());
                eprintln!("{error}");
            }
        }
    }

    if summary.checked == 0 && summary.malformed == 0 {
        bail!("no checksums found in {}", checksum_file.display());
    }

    if summary.failed > 0 || summary.malformed > 0 {
        bail!(
            "checksum verification failed: {} checked, {} failed, {} malformed",
            summary.checked,
            summary.failed,
            summary.malformed
        );
    }

    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::parse_checksum_line;
    use crate::command::Algorithm;
    use anyhow::Result;
    use std::path::PathBuf;

    #[test]
    fn parses_hfile_tab_format() -> Result<()> {
        let entry = parse_checksum_line(
            "c03905fcdab297513a620ec81ed46ca44ddb62d41cbbd83eb4a5a3592be26a69\tfile.txt",
            Algorithm::Sha256,
        )?
        .ok_or_else(|| anyhow::anyhow!("missing checksum entry"))?;

        assert_eq!(
            entry.digest,
            "c03905fcdab297513a620ec81ed46ca44ddb62d41cbbd83eb4a5a3592be26a69"
        );
        assert_eq!(entry.path, PathBuf::from("file.txt"));

        Ok(())
    }

    #[test]
    fn parses_gnu_escaped_format() -> Result<()> {
        let entry = parse_checksum_line(
            "\\c03905fcdab297513a620ec81ed46ca44ddb62d41cbbd83eb4a5a3592be26a69  file\\\\name.txt",
            Algorithm::Sha256,
        )?
        .ok_or_else(|| anyhow::anyhow!("missing checksum entry"))?;

        assert_eq!(entry.path, PathBuf::from("file\\name.txt"));

        Ok(())
    }
}
