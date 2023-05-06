use clap::Parser;
use hfile::command::Cli;

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
