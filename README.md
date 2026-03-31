# hfile

[![test & build](https://github.com/nbari/hfile/actions/workflows/build.yml/badge.svg)](https://github.com/nbari/hfile/actions/workflows/build.yml)
[![crates.io](https://img.shields.io/crates/v/hfile)](https://crates.io/crates/hfile)
[![coverage](https://codecov.io/gh/nbari/hfile/graph/badge.svg)](https://app.codecov.io/gh/nbari/hfile)
[![license](https://img.shields.io/github/license/nbari/hfile)](LICENSE)

`hfile` is a Rust CLI for generating file hashes, verifying checksum manifests,
and finding duplicate files.

For regular files, the default BLAKE3 path uses memory-mapped, multithreaded
hashing to stay close to `b3sum` throughput while keeping the output simple and
script-friendly.

## Features

- BLAKE3 by default for regular files
- Support for `md5`, `sha1`, `sha256`, `sha384`, and `sha512`
- Recursive hashing with `--path`
- Duplicate detection with `--duplicates`
- Manifest verification with `--check`
- Output that fits `hfile` and GNU checksum workflows

## Installation

Install from the repository:

```sh
cargo install --path .
```

Build a release binary locally:

```sh
cargo build --release --locked
```

Tagged releases publish `hfile.gz`, `.deb`, and `.rpm` artifacts on
[GitHub Releases](https://github.com/nbari/hfile/releases).

## Usage

```text
hfile [OPTIONS] [FILE]
```

Quick reference:

- `hfile FILE`: hash a single file
- `-a, --algorithm <ALGORITHM>`: select `blake`, `md5`, `sha1`, `sha256`,
  `sha384`, or `sha512`
- `-s, --size`: show the file size next to the digest
- `-p, --path <PATH>`: hash every file under a directory
- `-d, --duplicates`: print duplicate files found under `--path`
- `-c, --check <CHECK>`: verify checksums from a manifest file

Use `hfile --help` for the full CLI reference.

## Examples

Default BLAKE3 hash for a single file:

```sh
hfile tests/test-file
9a689455c65ca329fbcae5a1ae8725d88c7a6fbc82fd25bbcd9370ad9c272c50	tests/test-file
```

Show the hash and file size:

```sh
hfile -s tests/test-file
```

Use SHA-256:

```sh
hfile -a sha256 tests/test-file
```

Hash every file under a directory:

```sh
hfile -p tests
```

Print only duplicate files found under a directory:

```sh
hfile -d -p "$HOME"
```

Create a SHA-256 checksum file and verify it with GNU `sha256sum` or `hfile`:

```sh
hfile -a sha256 -p tests > SHA256SUMS
sha256sum -c SHA256SUMS
hfile -a sha256 -c SHA256SUMS
```

Do not use `--size` when creating checksum files; the extra size column is for
display only.

## Benchmarks

```sh
just compare-blake3
just compare-blake3-native
```

`just compare-blake3` benchmarks the portable release build against `b3sum` on
`tests/test-file` and a generated 256 MiB file. `just compare-blake3-native`
reruns the same comparison with `target-cpu=native` for local tuning.
