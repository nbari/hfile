use crate::cli::{actions::Action, commands, dispatch};
use anyhow::Result;

/// Main entry point for the CLI.
///
/// # Errors
///
/// Returns an error if argument parsing or dispatch fails.
pub fn start() -> Result<Action> {
    let matches = commands::new().get_matches();
    dispatch::handler(&matches)
}
