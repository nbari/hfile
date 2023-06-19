use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about = "hfile generates cryptographic hashes from files.", long_about = None)]
pub struct Args {
    #[arg(
        short = 'p',
        long = "path",
        help = "Create hash for all files under path"
    )]
    pub path: Option<String>,

    #[arg(short = 'a', long = "algorithm", value_enum, default_value_t=Algorithm::Blake)]
    pub algorithm: Algorithm,

    #[clap(required_unless_present("path"))]
    pub file: Option<String>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Algorithm {
    Md5,
    Sha1,
    Sha256,
    Sha384,
    Sha512,
    Blake,
}
