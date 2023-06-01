use clap::Parser;
use hfile::command::Args;

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
