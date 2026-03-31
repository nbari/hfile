use anyhow::Result;
use hfile::cli;

fn run() -> Result<()> {
    let action = cli::start()?;
    action.execute()
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
