//! Rust interface for fast and efficient image deduplication.
//! Leverages perceptual hashing algorithms to identify duplicate or visually similar images in a directory.

use anyhow::Error;
use image::imageops::FilterType;
use imgddcore::dedupe::*;
use imgddcore::validate::*;
use std::collections::HashMap;
use std::path::PathBuf;

/// Converts a string to a `FilterType`.
///
/// # Arguments
///
/// - `filter` - String specifying the filter type.
///     - **Options:** [`Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`]
///
/// # Returns
///
/// - A `FilterType` enum corresponding to the input string.
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

/// Selects a hashing algorithm.
///
/// # Arguments
///
/// - `algo` - String specifying the hashing algorithm.
///     - **Options:** [`aHash`, `mHash`, `dHash`, `pHash`, `wHash`]
///
/// # Returns
///
/// - A standardized `&'static str` representing the selected algorithm.
#[inline]
pub fn select_algo(algo: Option<&str>) -> &'static str {
    match algo.unwrap_or("dhash") {
        input if input.eq_ignore_ascii_case("dhash") => "dhash",
        input if input.eq_ignore_ascii_case("ahash") => "ahash",
        input if input.eq_ignore_ascii_case("mhash") => "mhash",
        input if input.eq_ignore_ascii_case("phash") => "phash",
        input if input.eq_ignore_ascii_case("whash") => "whash",
        other => panic!("Unsupported algorithm: {}", other),
    }
}

/// Calculates hashes for all images in a directory recursively.
///
/// # Arguments
///
/// - `path` - String representing the directory containing images.
/// - `filter` - String specifying the resize filter to use.
///     - **Options:** [`Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`]
///     - **Default:** "Triangle"
/// - `algo` - String specifying the hashing algorithm to use.
///     - **Options:** [`aHash`, `mHash`, `dHash`, `pHash`, `wHash`]
///     - **Default:** "dHash"
/// - `sort` - Boolean to determine if the hashes should be sorted.
///     - **Default:** false
/// - `hash_size` - Integer specifying the hash size to use for pHash.
///   The resulting hash will be hash_size^2 bits long. The value is
///   ignored for all hash methods other than pHash.
///     - **Default:**  8
///
/// # Returns
///
/// * A vector of tuples where each tuple contains a hash value and the corresponding file path.
///
/// # Usage
/// ```rust
/// use imgdd::*;
/// use std::path::PathBuf;
///
/// let result = hash(
///     PathBuf::from("path/to/images"),
///     Some("Triangle"), // Optional: default = "Triangle"
///     Some("dHash"),   // Optional: default = "dHash"
///     None,            // Optional: default = 8
///     Some(false),     // Optional: default = false
/// );
///
/// println!("{:#?}", result);
/// ```
pub fn hash(
    path: PathBuf,
    filter: Option<&str>,
    algo: Option<&str>,
    hash_size: Option<usize>,
    sort: Option<bool>,
) -> Result<Vec<(imgddcore::hashing::ImageHash, PathBuf)>, Error> {
    let validated_path = validate_path(&path)?;
    let filter_type = select_filter_type(filter);
    let selected_algo = select_algo(algo);

    let mut hash_paths = collect_hashes(&validated_path, filter_type, selected_algo, hash_size)?;

    // Optionally sort hashes
    if sort.unwrap_or(false) {
        sort_hashes(&mut hash_paths);
    }

    Ok(hash_paths)
}

/// Finds duplicate images in a directory.
///
/// # Arguments
///
/// - `path` - String representing the directory containing images.
/// - `filter` - String specifying the resize filter to use.
///     - **Options:** [`Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`]
///     - **Default:** "Triangle"
/// - `algo` - String specifying the hashing algorithm to use.
///     - **Options:** [`aHash`, `mHash`, `dHash`, `pHash`, `wHash`]
///     - **Default:** "dhash"
/// - `hash_size` - Integer specifying the hash size to use for pHash.
///   The resulting hash will be hash_size^2 bits long. The value is)
///   ignored for all hash methods other than pHash.
///     - **Default:**  8
/// - `remove` - Boolean indicating whether duplicate files should be removed.
///
/// # Returns
///
/// * A hashmap of hash values to lists of file paths.
///
/// # Usage
/// ```rust
/// use imgdd::*;
/// use std::path::PathBuf;
///
/// let result = dupes(
///     PathBuf::from("path/to/images"),
///     Some("Triangle"), // Optional: default = "Triangle"
///     Some("dHash"),   // Optional: default = "dHash"
///     None,            // Optional: default = 8
///     false,
/// );
///
/// println!("{:#?}", result);
/// ```
pub fn dupes(
    path: PathBuf,
    filter: Option<&str>,
    algo: Option<&str>,
    hash_size: Option<usize>,
    remove: bool,
) -> Result<HashMap<imgddcore::hashing::ImageHash, Vec<PathBuf>>, Error> {
    let validated_path = validate_path(&path)?;
    let filter_type = select_filter_type(filter);
    let selected_algo = select_algo(algo);

    let mut hash_paths = collect_hashes(&validated_path, filter_type, selected_algo, hash_size)?;
    sort_hashes(&mut hash_paths);

    Ok(find_duplicates(&hash_paths, remove)?)
}
