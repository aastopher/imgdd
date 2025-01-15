pub mod hashing;
pub mod normalize;
pub mod dedupe;
pub mod validate;

use validate::validate_path;
use dedupe::{collect_hashes, sort_hashes, find_duplicates};
use pyo3::prelude::*;
use image::imageops::FilterType;
use std::collections::HashMap;
use std::path::PathBuf;

#[inline]
fn select_filter_type(filter: Option<&str>) -> FilterType {
    match filter.unwrap_or("nearest") {
        ref f if f.eq_ignore_ascii_case("nearest") => FilterType::Nearest,
        ref f if f.eq_ignore_ascii_case("triangle") => FilterType::Triangle,
        ref f if f.eq_ignore_ascii_case("catmullrom") => FilterType::CatmullRom,
        ref f if f.eq_ignore_ascii_case("gaussian") => FilterType::Gaussian,
        ref f if f.eq_ignore_ascii_case("lanczos3") => FilterType::Lanczos3,
        other => panic!("Unsupported filter type: {}", other),
    }
}

#[inline]
fn select_algo(algo: Option<&str>) -> &'static str {
    match algo.unwrap_or("dhash") {
        input if input.eq_ignore_ascii_case("dhash") => "dhash",
        input if input.eq_ignore_ascii_case("ahash") => "ahash",
        input if input.eq_ignore_ascii_case("bhash") => "bhash",
        input if input.eq_ignore_ascii_case("mhash") => "mhash",
        input if input.eq_ignore_ascii_case("phash") => "phash",
        input if input.eq_ignore_ascii_case("whash") => "whash",
        other => panic!("Unsupported algorithm: {}", other),
    }
}

/// Returns a dictionary of image paths to their hex-formatted hashes.
#[pyfunction(signature = (path, filter = None, algo = None))]
fn hash(
    path: PathBuf,
    filter: Option<&str>,
    algo: Option<&str>,
) -> PyResult<HashMap<PathBuf, String>> {
    let validated_path = validate_path(&path)?;
    let filter_type = select_filter_type(filter);
    let algo = select_algo(algo);

    let hash_paths = collect_hashes(&validated_path, filter_type, &algo)?;

    Ok(hash_paths
        .into_iter()
        .map(|(hash, path)| (path, format!("{:x}", hash)))
        .collect())
}

/// Returns a dictionary of hex-formatted hashes to their duplicate paths.
#[pyfunction(signature = (path, filter = None, algo = None, remove = false))]
fn dupes(
    path: PathBuf,
    filter: Option<&str>,
    algo: Option<&str>,
    remove: bool,
) -> PyResult<HashMap<String, Vec<PathBuf>>> {
    let validated_path = validate_path(&path)?;
    let filter_type = select_filter_type(filter);
    let algo = select_algo(algo);

    let mut hash_paths = collect_hashes(&validated_path, filter_type, &algo)?;
    sort_hashes(&mut hash_paths);

    let duplicates = find_duplicates(&hash_paths, remove)?;

    Ok(duplicates
        .into_iter()
        .map(|(hash, paths)| (format!("{:x}", hash), paths))
        .collect())
}


#[pymodule]
fn imgdd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dupes, m)?)?;
    m.add_function(wrap_pyfunction!(hash, m)?)
}
