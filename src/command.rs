use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about = "hfile generates cryptographic hashes from files.", long_about = None)]
pub struct Cli {
    #[arg(
        short = 'p',
        long = "path",
        help = "Create hash for all files under path"
    )]
    path: Option<String>,

    #[arg(
        short = 'a',
        long = "algorithm",
        help = "Algorithm to use, default BLAKE3"
    )]
    algorithm: Option<String>,
}
