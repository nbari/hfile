mod run;

use crate::cli::commands::Algorithm;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    HashFile {
        file: String,
        algorithm: Algorithm,
        size: bool,
    },
    HashPath {
        path: String,
        algorithm: Algorithm,
        size: bool,
    },
    FindDuplicates {
        path: String,
        algorithm: Algorithm,
    },
    CheckChecksums {
        checksum_file: String,
        algorithm: Algorithm,
    },
}

impl Action {
    /// # Errors
    /// Returns an error if executing the selected action fails.
    pub fn execute(&self) -> Result<()> {
        run::execute(self)
    }
}
