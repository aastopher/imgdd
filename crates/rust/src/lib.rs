use corelib::dedupe::*;
use corelib::validate::*;
use image::imageops::FilterType;
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Error;

#[inline]
pub fn select_filter_type(filter: Option<&str>) -> FilterType {
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
pub fn select_algo(algo: Option<&str>) -> &'static str {
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


pub fn hash(
    path: PathBuf,
    filter: Option<&str>,
    algo: Option<&str>,
    sort: Option<bool>,
) -> Result<Vec<(u64, PathBuf)>, Error> {
    let validated_path = validate_path(&path)?;
    let filter_type = select_filter_type(filter);
    let algo = select_algo(algo);

    let mut hash_paths = collect_hashes(&validated_path, filter_type, algo)?;

    // Optionally sort hashes
    if sort.unwrap_or(false) {
        sort_hashes(&mut hash_paths);
    }

    Ok(hash_paths)
}


pub fn dupes(
    path: PathBuf,
    filter: Option<&str>,
    algo: Option<&str>,
    remove: bool,
) -> Result<HashMap<u64, Vec<PathBuf>>, Error> {
    let validated_path = validate_path(&path)?;
    let filter_type = select_filter_type(filter);
    let algo = select_algo(algo);

    let mut hash_paths = collect_hashes(&validated_path, filter_type, &algo)?;
    sort_hashes(&mut hash_paths);

    Ok(find_duplicates(&hash_paths, remove)?)
}