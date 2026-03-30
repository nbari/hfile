# Repository Guidelines

## Project Structure & Module Organization
`hfile` is a Rust CLI. Core modules live in `src/`: `hash.rs` for algorithms, `walkdir.rs` for scans and duplicate detection, and `command.rs` for the `clap` interface. `src/bin/hfile.rs` is the entry point. Integration tests live in `tests/tests.rs`, with `tests/test-file` as the fixture. CI workflows are under `.github/workflows/`.

## Build, Test, and Development Commands
- `cargo check`: fast compile check during development.
- `cargo run -- tests/test-file`: run the CLI locally against the sample fixture.
- `cargo run -- -d -p tests`: scan the `tests/` directory for duplicate files.
- `cargo test`: run the current integration suite and doc/unit test targets.
- `cargo fmt --all`: apply standard Rust formatting.
- `cargo fmt --all -- --check`: verify formatting exactly as CI does.
- `cargo clippy --all-targets --all-features`: required before submitting changes; `Cargo.toml` enforces the lint levels.
- `cargo build --release --locked`: produce the release binary used by the build workflow.
- `just compare-blake3`: benchmark the release binary against `b3sum` on `tests/test-file` and a generated 256 MiB file.
- `just compare-blake3-native`: rerun the benchmark with `target-cpu=native` for tuning comparisons.

## Coding Style & Naming Conventions
Use Rust 2024 idioms and let `rustfmt` own formatting; do not hand-tune spacing. Follow existing naming: modules and functions in `snake_case`, enums and structs in `CamelCase`, and concise CLI flags in `src/command.rs`. Prefer small, focused functions, return `anyhow::Result` for fallible helpers, and keep user-facing error printing at the CLI boundary. For public fallible functions, keep `# Errors` doc comments in place. The main optimization target is `hfile FILE`, so avoid changes that slow the default BLAKE3 regular-file path without benchmark evidence.

## Testing Guidelines
Add regression coverage in `tests/tests.rs` for behavior that changes. Existing tests use `hash_<algorithm>` names and validate deterministic hashes against `tests/test-file`; follow that pattern for new algorithms or fixtures. Changes to hashing, traversal, or duplicate reporting should ship with a test. Run `cargo test`, `cargo fmt --all -- --check`, and `cargo clippy --all-targets --all-features` before opening a PR. If you touch the BLAKE3 fast path, also run `just compare-blake3`.

## Commit & Pull Request Guidelines
Recent history favors short, direct subjects such as `cargo bump`, `tidy up README`, and version tags like `0.3.7`. Keep commits single-purpose and use concise imperative subjects. PRs should summarize behavior changes, list the commands you ran, and follow `.github/PULL_REQUEST_TEMPLATE.md` by confirming tests pass and targeting `develop` unless a maintainer asks for `main`.
