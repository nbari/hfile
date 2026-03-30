use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about = "hfile generates cryptographic hashes from files.", long_about = None)]
pub struct Args {
    #[clap(short = 'a', long = "algorithm", value_enum, default_value_t=Algorithm::Blake)]
    pub algorithm: Algorithm,

    #[clap(short = 's', long = "size", help = "Show size of file")]
    pub size: bool,

    #[clap(required_unless_present_any(["path", "duplicates", "check"]))]
    pub file: Option<String>,

    #[clap(
        short = 'd',
        long = "duplicates",
        help = "Find duplicates",
        requires = "path"
    )]
    pub duplicates: bool,

    #[clap(
        short = 'p',
        long = "path",
        help = "Create hash for all files under path"
    )]
    pub path: Option<String>,

    #[clap(
        short = 'c',
        long = "check",
        help = "Read checksums from file and verify them",
        conflicts_with_all = ["path", "duplicates", "size", "file"]
    )]
    pub check: Option<String>,
}

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum Algorithm {
    Md5,
    Sha1,
    Sha256,
    Sha384,
    Sha512,
    Blake,
}

impl Algorithm {
    #[must_use]
    pub const fn digest_len(self) -> usize {
        match self {
            Self::Md5 => 32,
            Self::Sha1 => 40,
            Self::Sha256 | Self::Blake => 64,
            Self::Sha384 => 96,
            Self::Sha512 => 128,
        }
    }
}
