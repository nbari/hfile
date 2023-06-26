# hfile

Hfile is a command-line interface (CLI) tool for generating cryptographic hashes
from files while also facilitating the identification of duplicates.

Supported alrotihms:
* Blake3 (default)
* md5
* sha1
* sha256
* sha384
* sha512

Example:

    $ hfile file
    9a689455c65ca329fbcae5a1ae8725d88c7a6fbc82fd25bbcd9370ad9c272c50        test-file

If need also the size of the file:

    $ hfile -s <file>
    9a689455c65ca329fbcae5a1ae8725d88c7a6fbc82fd25bbcd9370ad9c272c50        test-file 44B

To recursively get hash of all files within a directory:

    $ hfile -p $HOME
    <hash> <file path>

Finding duplicates:

    $ hfile -d -p $HOME
    will only print duplicates found
