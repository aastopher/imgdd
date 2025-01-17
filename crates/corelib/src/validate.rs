use anyhow::{anyhow, Result};
use std::path::PathBuf;

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
