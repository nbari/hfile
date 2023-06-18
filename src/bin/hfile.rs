use clap::Parser;
use hfile::command::{Algorithm, Args};
use hfile::hash;

fn main() {
    let args = Args::parse();

    let hash = match args.algorithm {
        Algorithm::Md5 => hash::md5(&args.file),
        Algorithm::Sha1 => hash::sha1(&args.file),
        Algorithm::Sha256 => hash::sha256(&args.file),
        Algorithm::Sha384 => hash::sha384(&args.file),
        Algorithm::Sha512 => hash::sha512(&args.file),
        Algorithm::Blake => hash::blake3(&args.file),
    };

    match hash {
        Ok(h) => println!("{h}\t{}", args.file),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
