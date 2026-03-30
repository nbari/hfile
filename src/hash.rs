use crate::command::Algorithm;
use anyhow::{Context, Result, anyhow};
use ring::digest::{Context as DigestContext, SHA1_FOR_LEGACY_USE_ONLY, SHA256, SHA384, SHA512};
use std::{fs, io::Read, path::Path};

const BUFFER_SIZE: usize = 1024 * 1024;
const BLAKE3_RAYON_MIN_BYTES: u64 = 128 * 1024;
const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

#[derive(Copy, Clone)]
enum Blake3Mode {
    AlwaysRayon,
    Adaptive,
}

fn read_file_chunks(file_path: &Path, mut on_chunk: impl FnMut(&[u8])) -> Result<()> {
    let mut file = std::fs::File::open(file_path)?;
    let mut buf = vec![0_u8; BUFFER_SIZE].into_boxed_slice();

    loop {
        let size = file.read(&mut buf)?;
        if size == 0 {
            break;
        }

        let chunk = buf
            .get(..size)
            .ok_or_else(|| anyhow!("read size exceeded buffer length"))?;
        on_chunk(chunk);
    }

    Ok(())
}

fn digest_file(algo: Algorithm, file_path: &Path, blake3_mode: Blake3Mode) -> Result<Vec<u8>> {
    let hash = match algo {
        Algorithm::Md5 => md5_digest(file_path)?.to_vec(),
        Algorithm::Sha1 => sha1_digest(file_path)?,
        Algorithm::Sha256 => sha256_digest(file_path)?,
        Algorithm::Sha384 => sha384_digest(file_path)?,
        Algorithm::Sha512 => sha512_digest(file_path)?,
        Algorithm::Blake => blake3_digest(file_path, blake3_mode)?.to_vec(),
    };

    Ok(hash)
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn hash_file(algo: Algorithm, file_path: &Path) -> Result<String> {
    let hash = digest_file(algo, file_path, Blake3Mode::AlwaysRayon)
        .with_context(|| format!("failed to hash {}", file_path.display()))?;
    Ok(write_hex_bytes(&hash))
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub(crate) fn hash_file_for_walk(algo: Algorithm, file_path: &Path) -> Result<String> {
    let hash = digest_file(algo, file_path, Blake3Mode::Adaptive)
        .with_context(|| format!("failed to hash {}", file_path.display()))?;
    Ok(write_hex_bytes(&hash))
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub(crate) fn hash_file_bytes_for_walk(algo: Algorithm, file_path: &Path) -> Result<Vec<u8>> {
    digest_file(algo, file_path, Blake3Mode::Adaptive)
        .with_context(|| format!("failed to hash {}", file_path.display()))
}

fn blake3_digest(file_path: &Path, mode: Blake3Mode) -> Result<[u8; blake3::OUT_LEN]> {
    let mut hasher = blake3::Hasher::new();

    match mode {
        Blake3Mode::AlwaysRayon => {
            hasher.update_mmap_rayon(file_path)?;
        }
        Blake3Mode::Adaptive => {
            let metadata = fs::metadata(file_path)?;
            if metadata.len() >= BLAKE3_RAYON_MIN_BYTES {
                hasher.update_mmap_rayon(file_path)?;
            } else {
                hasher.update_mmap(file_path)?;
            }
        }
    }

    Ok(*hasher.finalize().as_bytes())
}

fn md5_digest(file_path: &Path) -> Result<[u8; 16]> {
    let mut context = md5::Context::new();
    read_file_chunks(file_path, |chunk| context.consume(chunk))?;
    Ok(context.compute().0)
}

fn sha1_digest(file_path: &Path) -> Result<Vec<u8>> {
    let mut context = DigestContext::new(&SHA1_FOR_LEGACY_USE_ONLY);
    read_file_chunks(file_path, |chunk| context.update(chunk))?;
    Ok(context.finish().as_ref().to_vec())
}

fn sha256_digest(file_path: &Path) -> Result<Vec<u8>> {
    let mut context = DigestContext::new(&SHA256);
    read_file_chunks(file_path, |chunk| context.update(chunk))?;
    Ok(context.finish().as_ref().to_vec())
}

fn sha384_digest(file_path: &Path) -> Result<Vec<u8>> {
    let mut context = DigestContext::new(&SHA384);
    read_file_chunks(file_path, |chunk| context.update(chunk))?;
    Ok(context.finish().as_ref().to_vec())
}

fn sha512_digest(file_path: &Path) -> Result<Vec<u8>> {
    let mut context = DigestContext::new(&SHA512);
    read_file_chunks(file_path, |chunk| context.update(chunk))?;
    Ok(context.finish().as_ref().to_vec())
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn blake3(file_path: &Path) -> Result<String> {
    let hash = blake3_digest(file_path, Blake3Mode::AlwaysRayon)?;
    Ok(write_hex_bytes(&hash))
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn md5(file_path: &Path) -> Result<String> {
    let hash = md5_digest(file_path)?;
    Ok(write_hex_bytes(&hash))
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn sha1(file_path: &Path) -> Result<String> {
    let hash = sha1_digest(file_path)?;
    Ok(write_hex_bytes(&hash))
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn sha256(file_path: &Path) -> Result<String> {
    let hash = sha256_digest(file_path)?;
    Ok(write_hex_bytes(&hash))
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn sha384(file_path: &Path) -> Result<String> {
    let hash = sha384_digest(file_path)?;
    Ok(write_hex_bytes(&hash))
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn sha512(file_path: &Path) -> Result<String> {
    let hash = sha512_digest(file_path)?;
    Ok(write_hex_bytes(&hash))
}

#[must_use]
pub fn write_hex_bytes(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        let high = HEX_DIGITS
            .get(usize::from(byte >> 4))
            .copied()
            .unwrap_or(b'0');
        let low = HEX_DIGITS
            .get(usize::from(byte & 0x0f))
            .copied()
            .unwrap_or(b'0');
        s.push(char::from(high));
        s.push(char::from(low));
    }
    s
}
