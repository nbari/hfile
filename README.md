# hfile

**hfile** is a command-line interface (CLI) tool for generating cryptographic hashes
from files while also facilitating the identification of duplicates.

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
  -d, --duplicates             Find duplicates
  -p, --path <PATH>            Create hash for all files under path
  -h, --help                   Print help
  -V, --version                Print version
```

Example:

    $ hfile test-file
    9a689455c65ca329fbcae5a1ae8725d88c7a6fbc82fd25bbcd9370ad9c272c50    test-file

If need also the size of the file:

    $ hfile -s test-file
    9a689455c65ca329fbcae5a1ae8725d88c7a6fbc82fd25bbcd9370ad9c272c50    test-file    44B

To recursively get hash of all files within a directory:

    $ hfile -p $HOME
    <hash> <file path>

Finding duplicates:

    $ hfile -d -p $HOME
    will only print duplicates found
