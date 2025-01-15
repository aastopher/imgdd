use pyo3::exceptions::PyValueError;
use pyo3::PyResult;
use log::{debug, error};
use std::path::PathBuf;

#[inline]
pub fn validate_path(path: &PathBuf) -> PyResult<&PathBuf> {

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

    debug!("Path is a directory: {}", path.display());
    Ok(&path)
}
