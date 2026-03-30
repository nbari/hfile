use anyhow::{Result, anyhow};
use ring::digest::{Context, SHA1_FOR_LEGACY_USE_ONLY, SHA256, SHA384, SHA512};
use std::{io::Read, path::Path};

const BUFFER_SIZE: usize = 1024 * 1024;
const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

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

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn blake3(file_path: &Path) -> Result<String> {
    let mut hasher = blake3::Hasher::new();
    hasher.update_mmap_rayon(file_path)?;
    Ok(hasher.finalize().to_hex().to_string())
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn md5(file_path: &Path) -> Result<String> {
    let mut context = md5::Context::new();
    read_file_chunks(file_path, |chunk| context.consume(chunk))?;
    Ok(write_hex_bytes(context.compute().as_ref()))
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn sha1(file_path: &Path) -> Result<String> {
    let mut context = Context::new(&SHA1_FOR_LEGACY_USE_ONLY);
    read_file_chunks(file_path, |chunk| context.update(chunk))?;
    Ok(write_hex_bytes(context.finish().as_ref()))
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn sha256(file_path: &Path) -> Result<String> {
    let mut context = Context::new(&SHA256);
    read_file_chunks(file_path, |chunk| context.update(chunk))?;
    Ok(write_hex_bytes(context.finish().as_ref()))
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn sha384(file_path: &Path) -> Result<String> {
    let mut context = Context::new(&SHA384);
    read_file_chunks(file_path, |chunk| context.update(chunk))?;
    Ok(write_hex_bytes(context.finish().as_ref()))
}

/// # Errors
/// Returns an error if the file cannot be opened or read.
pub fn sha512(file_path: &Path) -> Result<String> {
    let mut context = Context::new(&SHA512);
    read_file_chunks(file_path, |chunk| context.update(chunk))?;
    Ok(write_hex_bytes(context.finish().as_ref()))
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
