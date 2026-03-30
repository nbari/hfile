# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0] - 2026-03-30

### Changed
- Migrated the crate to Rust 2024 and aligned the codebase with the stricter lint configuration in `Cargo.toml`.
- Reworked the default single-file BLAKE3 path to use memory-mapped, multithreaded hashing via the `blake3` crate `mmap` and `rayon` features.
- Removed the Tokio-based CLI/runtime path and replaced directory hashing concurrency with a bounded standard-thread worker queue.
- Increased streaming read buffers for non-BLAKE3 algorithms and reduced redundant filesystem metadata lookups in the single-file path.

### Fixed
- Preserved duplicate filenames containing spaces by storing duplicate paths as structured `Vec<PathBuf>` collections instead of whitespace-joined strings.
- Updated tests to avoid `unwrap`/`expect` patterns forbidden by the current lint policy.

### Added
- Added release packaging support for GitHub Releases with compressed binaries, `.deb`, and `.rpm` artifacts.
- Added local comparison tooling with `just compare-blake3` and `just compare-blake3-native` for `hfile` vs `b3sum` throughput checks.
- Added regression coverage for duplicate detection with filenames containing spaces.
