# hfile

**hfile** is a command-line interface (CLI) tool for generating cryptographic hashes
from files while also facilitating the identification of duplicates.

For regular files, the default BLAKE3 path uses memory-mapped, multithreaded hashing
to stay close to `b3sum` throughput while keeping the CLI output unchanged.

Supported algorithms:
* Blake3 (default)
* md5
* sha1
* sha256
* sha384
* sha512

Current options:

```sh

Usage: hfile [OPTIONS] [FILE]

Arguments:
  [FILE]

Options:
  -a, --algorithm <ALGORITHM>  [default: blake] [possible values: md5, sha1, sha256, sha384, sha512, blake]
  -s, --size                   Show size of file
  -c, --check <CHECK>          Read checksums from file and verify them
  -d, --duplicates             Find duplicates
  -p, --path <PATH>            Create hash for all files under path
  -h, --help                   Print help
  -V, --version                Print version
```

## Examples

Default BLAKE3 hash for a single file:

```sh
hfile tests/test-file
9a689455c65ca329fbcae5a1ae8725d88c7a6fbc82fd25bbcd9370ad9c272c50	tests/test-file
```

Show the hash and file size:

```sh
hfile -s tests/test-file
9a689455c65ca329fbcae5a1ae8725d88c7a6fbc82fd25bbcd9370ad9c272c50	tests/test-file	44 B
```

Use a different algorithm:

```sh
hfile -a sha256 tests/test-file
c03905fcdab297513a620ec81ed46ca44ddb62d41cbbd83eb4a5a3592be26a69	tests/test-file
```

Hash every file under a directory:

```sh
hfile -p tests
```

Print only duplicate files found under a directory:

```sh
hfile -d -p "$HOME"
```

Create a SHA-256 checksum file that GNU `sha256sum -c` can verify:

```sh
hfile -a sha256 tests/test-file > SHA256SUMS
sha256sum -c SHA256SUMS
```

Create and verify checksums for all files under a directory:

```sh
hfile -a sha256 -p tests > SHA256SUMS
sha256sum -c SHA256SUMS
```

Verify the same checksum file with `hfile`:

```sh
hfile -a sha256 -c SHA256SUMS
```

Do not use `-s` when creating checksum files, since the extra size column is for display only.

Performance comparison:

```sh
just compare-blake3
just compare-blake3-native
```

`compare-blake3` benchmarks the portable release build against `b3sum` on the fixture
file and a generated 256 MiB regular file. `compare-blake3-native` reruns the same
comparison with `target-cpu=native` for local headroom checks.
