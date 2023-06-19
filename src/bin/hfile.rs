use clap::Parser;
use hfile::command::{Algorithm, Args};
use hfile::hash;
use hfile::walkdir;
use std::fs;

fn main() {
    let args = Args::parse();

    match args.path {
        None => {
            if let Some(file) = &args.file {
                match fs::metadata(file) {
                    Ok(metadata) => {
                        if metadata.is_dir() {
                            eprintln!("Use option -p to pass a directory");
                            std::process::exit(1);
                        } else {
                            let hash = match args.algorithm {
                                Algorithm::Md5 => hash::md5(file),
                                Algorithm::Sha1 => hash::sha1(file),
                                Algorithm::Sha256 => hash::sha256(file),
                                Algorithm::Sha384 => hash::sha384(file),
                                Algorithm::Sha512 => hash::sha512(file),
                                Algorithm::Blake => hash::blake3(file),
                            };

                            match hash {
                                Ok(h) => println!("{h}\t{}", file),
                                Err(e) => {
                                    eprintln!("{e}");
                                    std::process::exit(1);
                                }
                            }
                        }
                    }

                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                }
            }
        }
        Some(ref s) => match walkdir::read(s) {
            Ok(s) => {
                println!("{s}")
            }
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        },
    }
}
