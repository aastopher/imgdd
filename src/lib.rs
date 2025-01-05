mod hashing;
mod normalize;
mod dedupe;
mod validate;

use validate::validate_path;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use image::imageops::FilterType;
use std::path::PathBuf;
use crate::dedupe::{collect_hashes, find_duplicates};


#[pyfunction(signature = (path, filter = None, algo = None, remove = false))]
fn proc(
    path: PathBuf,
    filter: Option<&str>,
    algo: Option<&str>,
    remove: bool,
) -> PyResult<Py<PyDict>> {
    // Validate the provided path
    let validated_path = validate_path(Some(path)).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Validation error: {}", e))
    })?;

    // Use default if none provided; panic if invalid
    let filter_type = match filter.unwrap_or("nearest").to_lowercase().as_str() {
        "nearest" => FilterType::Nearest,
        "triangle" => FilterType::Triangle,
        "catmullrom" => FilterType::CatmullRom,
        "gaussian" => FilterType::Gaussian,
        "lanczos3" => FilterType::Lanczos3,
        other => panic!("Unsupported filter type: {}", other),
    };

    let algo = algo.unwrap_or("dhash").to_lowercase();
    if algo != "dhash" {
        panic!("Unsupported hashing algorithm: {}", algo);
    }

    let hashes_with_paths = collect_hashes(&validated_path, filter_type, &algo).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Error collecting hashes: {}", e))
    })?;

    let duplicates = find_duplicates(&hashes_with_paths, remove).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Error finding duplicates: {}", e))
    })?;

    Python::with_gil(|py| {
        let result = PyDict::new(py);
        for (hash, paths) in duplicates {
            result.set_item(hash, paths)?;
        }
        Ok(result.into())
    })
}


#[pymodule]
fn imgdd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(proc, m)?)
}