use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use log::{debug, error};
use std::path::PathBuf;

pub fn validate_path(path: Option<PathBuf>) -> PyResult<PathBuf> {
    let path = path.unwrap_or_else(|| {
        std::env::current_dir().expect("Failed to get current directory")
    });

    if !path.exists() {
        error!("Path does not exist: {}", path.display());
        return Err(PyValueError::new_err(format!(
            "Path does not exist: {}",
            path.display()
        )));
    } else {
        debug!("Path exists: {}", path.display());
    }

    if !path.is_dir() {
        error!("Path is not a directory: {}", path.display());
        return Err(PyValueError::new_err(format!(
            "Path is not a directory: {}",
            path.display()
        )));
    } else {
        debug!("Path is a directory: {}", path.display());
    }

    Ok(path)
}
