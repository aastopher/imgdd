use pyo3::exceptions::PyValueError;
use pyo3::PyResult;
use log::{debug, error};
use std::path::PathBuf;

#[inline]
pub fn validate_path(path: &PathBuf) -> PyResult<&PathBuf> {
    if !path.exists() {
        let message = format!("Path does not exist: {}", path.display());
        error!("{}", message);
        return Err(PyValueError::new_err(message));
    } else {
        debug!("Path exists: {}", path.display());
    }

    if !path.is_dir() {
        let message = format!("Path is not a directory: {}", path.display());
        error!("{}", message);
        return Err(PyValueError::new_err(message));
    } else {
        debug!("Path is a directory: {}", path.display());
    }

    debug!("Path is a directory: {}", path.display());
    Ok(&path)
}
