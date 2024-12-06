use anyhow::Result;
use log::{debug, error};
use std::path::PathBuf;

/// Retrieve and validate the path argument
pub fn validate_path(path: Option<PathBuf>) -> Result<PathBuf> {
    let path = path.unwrap_or_else(|| {
        std::env::current_dir().expect("Failed to get current directory")
    });

    if !path.exists() {
        error!("Path does not exist: {}", path.display());
        return Err(anyhow::anyhow!("Path does not exist: {}", path.display()));
    } else {
        debug!("Path exists: {}", path.display());
    }

    if !path.is_dir() {
        error!("Path is not a directory: {}", path.display());
        return Err(anyhow::anyhow!("Path is not a directory: {}", path.display()));
    } else {
        debug!("Path is a directory: {}", path.display());
    }

    Ok(path)
}
