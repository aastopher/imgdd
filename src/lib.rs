pub mod hashing;
pub mod normalize;
pub mod dedupe;
pub mod validate;

use validate::validate_path;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use image::imageops::FilterType;
use std::path::PathBuf;
use crate::dedupe::{collect_hashes, sort_hashes, find_duplicates};


#[pyfunction(signature = (path, filter = None, algo = None, remove = false))]
fn proc(
    path: PathBuf,
    filter: Option<&str>,
    algo: Option<&str>,
    remove: bool,
) -> PyResult<Py<PyDict>> {
    // Validate the provided path
    let validated_path = validate_path(Some(path))?;

    // Validate filter type; if none use "nearest"
    let filter_type = match filter.unwrap_or("nearest") {
        ref f if f.eq_ignore_ascii_case("nearest") => FilterType::Nearest,
        ref f if f.eq_ignore_ascii_case("triangle") => FilterType::Triangle,
        ref f if f.eq_ignore_ascii_case("catmullrom") => FilterType::CatmullRom,
        ref f if f.eq_ignore_ascii_case("gaussian") => FilterType::Gaussian,
        ref f if f.eq_ignore_ascii_case("lanczos3") => FilterType::Lanczos3,
        other => panic!("Unsupported filter type: {}", other),
    };
    
    // Validate algorithm; if none use "dhash"
    let algo = match algo.unwrap_or("dhash") {
        input if input.eq_ignore_ascii_case("dhash") => "dhash",
        input if input.eq_ignore_ascii_case("ahash") => "ahash",
        input if input.eq_ignore_ascii_case("bhash") => "bhash",
        input if input.eq_ignore_ascii_case("mhash") => "mhash",
        input if input.eq_ignore_ascii_case("phash") => "phash",
        input if input.eq_ignore_ascii_case("whash") => "whash",
        other => panic!("Unsupported algorithm: {}", other),
    };

    // Collect hashes and find duplicates
    let mut hash_paths = collect_hashes(&validated_path, filter_type, &algo)?;
    sort_hashes(&mut hash_paths);
    let duplicates = find_duplicates(&hash_paths, remove)?;

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