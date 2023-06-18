use anyhow::Result;
use ring::digest::{Context, SHA1_FOR_LEGACY_USE_ONLY, SHA256, SHA384, SHA512};
use std::fmt::Write;
use std::io::Read;

pub fn blake3(file_path: &str) -> Result<String> {
    let mut file = std::fs::File::open(file_path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buf = [0_u8; 65536];
    while let Ok(size) = file.read(&mut buf[..]) {
        if size == 0 {
            break;
        }
        hasher.update(&buf[0..size]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

pub fn md5(file_path: &str) -> Result<String> {
    let mut file = std::fs::File::open(file_path)?;
    let mut context = md5::Context::new();
    let mut buf = [0_u8; 65536];
    while let Ok(size) = file.read(&mut buf[..]) {
        if size == 0 {
            break;
        }
        context.consume(&buf[0..size]);
    }
    Ok(write_hex_bytes(context.compute().as_ref()))
}

pub fn sha1(file_path: &str) -> Result<String> {
    let mut file = std::fs::File::open(file_path)?;
    let mut context = Context::new(&SHA1_FOR_LEGACY_USE_ONLY);
    let mut buf = [0_u8; 65536];
    while let Ok(size) = file.read(&mut buf[..]) {
        if size == 0 {
            break;
        }
        context.update(&buf[0..size]);
    }
    Ok(write_hex_bytes(context.finish().as_ref()))
}

pub fn sha256(file_path: &str) -> Result<String> {
    let mut file = std::fs::File::open(file_path)?;
    let mut context = Context::new(&SHA256);
    let mut buf = [0_u8; 65536];
    while let Ok(size) = file.read(&mut buf[..]) {
        if size == 0 {
            break;
        }
        context.update(&buf[0..size]);
    }
    Ok(write_hex_bytes(context.finish().as_ref()))
}

pub fn sha384(file_path: &str) -> Result<String> {
    let mut file = std::fs::File::open(file_path)?;
    let mut context = Context::new(&SHA384);
    let mut buf = [0_u8; 65536];
    while let Ok(size) = file.read(&mut buf[..]) {
        if size == 0 {
            break;
        }
        context.update(&buf[0..size]);
    }
    Ok(write_hex_bytes(context.finish().as_ref()))
}

pub fn sha512(file_path: &str) -> Result<String> {
    let mut file = std::fs::File::open(file_path)?;
    let mut context = Context::new(&SHA512);
    let mut buf = [0_u8; 65536];
    while let Ok(size) = file.read(&mut buf[..]) {
        if size == 0 {
            break;
        }
        context.update(&buf[0..size]);
    }
    Ok(write_hex_bytes(context.finish().as_ref()))
}

pub fn write_hex_bytes(bytes: &[u8]) -> String {
    let mut s = String::new();
    for byte in bytes {
        write!(&mut s, "{:02x}", byte).expect("Unable to write");
    }
    s
}
