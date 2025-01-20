use anyhow::{anyhow, Result};
use std::path::PathBuf;


/// Validates a given path to ensure it exists and is a directory.
///
/// This function checks whether the provided path exists and is a directory,
/// returning an error if either condition is not met.
///
/// # Arguments
///
/// * `path` - A reference to a `PathBuf` representing the path to validate.
///
/// # Returns
///
/// * `Ok(&PathBuf)` - A reference to the validated `PathBuf` if the path exists and is a directory.
/// * `Err(anyhow::Error)` - An error indicating why the path is invalid.
#[inline]
pub fn validate_path(path: &PathBuf) -> Result<&PathBuf> {
    if !path.exists() {
        let message = format!("Path does not exist: {}", path.display());
        return Err(anyhow!(message));
    }

    if !path.is_dir() {
        let message = format!("Path is not a directory: {}", path.display());
        return Err(anyhow!(message));
    }

    Ok(path)
}
