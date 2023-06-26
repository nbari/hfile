use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about = "hfile generates cryptographic hashes from files.", long_about = None)]
pub struct Args {
    #[clap(short = 'a', long = "algorithm", value_enum, default_value_t=Algorithm::Blake)]
    pub algorithm: Algorithm,

    #[clap(short = 's', long = "size", help = "Show size of file")]
    pub size: bool,

    #[clap(required_unless_present_any(["path", "duplicates"]))]
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
