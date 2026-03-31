use crate::cli::{actions::Action, commands::Algorithm};
use anyhow::{Result, anyhow};
use clap::ArgMatches;

/// # Errors
/// Returns an error if required arguments are missing from the parsed matches.
pub fn handler(matches: &ArgMatches) -> Result<Action> {
    let algorithm = matches
        .get_one::<Algorithm>("algorithm")
        .copied()
        .ok_or_else(|| anyhow!("missing algorithm"))?;
    let size = matches.get_flag("size");

    if let Some(checksum_file) = matches.get_one::<String>("check") {
        return Ok(Action::CheckChecksums {
            checksum_file: checksum_file.clone(),
            algorithm,
        });
    }

    if matches.get_flag("duplicates") {
        let path = matches
            .get_one::<String>("path")
            .ok_or_else(|| anyhow!("missing path"))?;
        return Ok(Action::FindDuplicates {
            path: path.clone(),
            algorithm,
        });
    }

    if let Some(path) = matches.get_one::<String>("path") {
        return Ok(Action::HashPath {
            path: path.clone(),
            algorithm,
            size,
        });
    }

    if let Some(file) = matches.get_one::<String>("file") {
        return Ok(Action::HashFile {
            file: file.clone(),
            algorithm,
            size,
        });
    }

    Err(anyhow!("no action matched"))
}

#[cfg(test)]
mod tests {
    use super::handler;
    use crate::cli::{
        actions::Action,
        commands::{self, Algorithm},
    };
    use anyhow::Result;

    #[test]
    fn dispatches_hash_file() -> Result<()> {
        let matches = commands::new().try_get_matches_from(["hfile", "tests/test-file"])?;

        assert_eq!(
            handler(&matches)?,
            Action::HashFile {
                file: String::from("tests/test-file"),
                algorithm: Algorithm::Blake,
                size: false,
            }
        );

        Ok(())
    }

    #[test]
    fn dispatches_hash_path() -> Result<()> {
        let matches =
            commands::new().try_get_matches_from(["hfile", "-a", "sha256", "-s", "-p", "tests"])?;

        assert_eq!(
            handler(&matches)?,
            Action::HashPath {
                path: String::from("tests"),
                algorithm: Algorithm::Sha256,
                size: true,
            }
        );

        Ok(())
    }

    #[test]
    fn dispatches_find_duplicates() -> Result<()> {
        let matches = commands::new().try_get_matches_from(["hfile", "-d", "-p", "tests"])?;

        assert_eq!(
            handler(&matches)?,
            Action::FindDuplicates {
                path: String::from("tests"),
                algorithm: Algorithm::Blake,
            }
        );

        Ok(())
    }

    #[test]
    fn dispatches_checksum_verification() -> Result<()> {
        let matches =
            commands::new().try_get_matches_from(["hfile", "-a", "sha256", "-c", "SHA256SUMS"])?;

        assert_eq!(
            handler(&matches)?,
            Action::CheckChecksums {
                checksum_file: String::from("SHA256SUMS"),
                algorithm: Algorithm::Sha256,
            }
        );

        Ok(())
    }
}
