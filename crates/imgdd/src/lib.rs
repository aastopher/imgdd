//! Rust interface for fast and efficient image deduplication.
//! Leverages perceptual hashing algorithms to identify duplicate or visually similar images in a directory.

use anyhow::Error;
use imgddcore::dedupe::{collect_hashes, find_duplicates, sort_hashes};
use imgddcore::utils::{select_algo, select_filter_type};
use imgddcore::validate::validate_path;
use std::collections::HashMap;
use std::path::PathBuf;

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
///     Some(false),     // Optional: default = false
/// );
///
/// println!("{:#?}", result);
/// ```
pub fn hash(
    path: PathBuf,
    filter: Option<&str>,
    algo: Option<&str>,
    sort: Option<bool>,
) -> Result<Vec<(u64, PathBuf)>, Error> {
    let validated_path = validate_path(&path)?;
    let filter_type = select_filter_type(filter);
    let selected_algo = select_algo(algo);

    let mut hash_paths = collect_hashes(validated_path, filter_type, selected_algo)?;

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
///     false,
/// );
///
/// println!("{:#?}", result);
/// ```
pub fn dupes(
    path: PathBuf,
    filter: Option<&str>,
    algo: Option<&str>,
    remove: bool,
) -> Result<HashMap<u64, Vec<PathBuf>>, Error> {
    let validated_path = validate_path(&path)?;
    let filter_type = select_filter_type(filter);
    let selected_algo = select_algo(algo);

    let mut hash_paths = collect_hashes(validated_path, filter_type, selected_algo)?;
    sort_hashes(&mut hash_paths);

    // Ok(find_duplicates(&hash_paths, remove)?)
    find_duplicates(&hash_paths, remove)
}
