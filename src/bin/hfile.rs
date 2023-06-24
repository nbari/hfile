use clap::Parser;
use hfile::{
    command::{Algorithm, Args},
    hash, walkdir,
};
use std::{fs, path::Path};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.path {
        None => {
            if let Some(args_file) = &args.file {
                match fs::metadata(args_file) {
                    Ok(metadata) => {
                        if metadata.is_dir() {
                            eprintln!("Use option -p to pass a directory");
                            std::process::exit(1);
                        } else {
                            let file = Path::new(args_file);
                            let hash = match args.algorithm {
                                Algorithm::Md5 => hash::md5(file),
                                Algorithm::Sha1 => hash::sha1(file),
                                Algorithm::Sha256 => hash::sha256(file),
                                Algorithm::Sha384 => hash::sha384(file),
                                Algorithm::Sha512 => hash::sha512(file),
                                Algorithm::Blake => hash::blake3(file),
                            };

                            match hash {
                                Ok(h) => println!("{h}\t{}", file.display()),
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
        Some(ref s) => match walkdir::read(s, args.algorithm).await {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        },
    }
}
