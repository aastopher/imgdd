use crate::hashing::ImageHash;
use crate::normalize;
use image::imageops::FilterType;
use image::{DynamicImage, ImageReader};
use rayon::prelude::*;
use walkdir::WalkDir;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use anyhow::{anyhow, Result};
use anyhow::Error;

/// Collects hashes for all image files in a directory recursively.
///
/// # Arguments
///
/// * `path` - The directory containing images to process.
/// * `filter` - The resize filter to use. 
///              Options: `Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`.
/// * `algo` - The hashing algorithm to use. 
///              Options: `dhash`, `ahash`, `mhash`, `phash`, `whash`.
///
/// # Returns
///
/// * A vector of tuples containing the hash value and the corresponding file path.
pub fn collect_hashes(
    path: &PathBuf,
    filter: FilterType,
    algo: &str,
) -> Result<Vec<(u64, PathBuf)>, Error> {
    let files: Vec<PathBuf> = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().to_path_buf())
        .collect();

        let hash_paths: Vec<(u64, PathBuf)> = files
        .par_iter()
        .filter_map(|file_path| {
            match open_image(file_path) {
                Ok(image) => {
                    let hash = match algo {
                        "dhash" => {
                            let normalized = normalize::proc(&image, filter, 9, 8).ok()?;
                            ImageHash::dhash(&normalized).ok()?.get_hash()
                        }
                        "ahash" => {
                            let normalized = normalize::proc(&image, filter, 8, 8).ok()?;
                            ImageHash::ahash(&normalized).ok()?.get_hash()
                        }
                        "mhash" => {
                            let normalized = normalize::proc(&image, filter, 8, 8).ok()?;
                            ImageHash::mhash(&normalized).ok()?.get_hash()
                        }
                        "phash" => {
                            let normalized = normalize::proc(&image, filter, 32, 32).ok()?;
                            ImageHash::phash(&normalized).ok()?.get_hash()
                        }
                        "whash" => {
                            let normalized = normalize::proc(&image, filter, 8, 8).ok()?;
                            ImageHash::whash(&normalized).ok()?.get_hash()
                        }
                        _ => panic!("Unsupported hashing algorithm: {}", algo),
                    };
                    Some((hash, file_path.clone()))
                }
                Err(e) => {
                    eprintln!("Failed to open image {}: {}", file_path.display(), e);
                    None
                }
            }
        })
        .collect();

    Ok(hash_paths)
}

/// Sorts a vector of hashes by hash value.
///
/// # Arguments
///
/// * `hash_paths` - A mutable reference to a vector of hash-path tuples.
#[inline]
pub fn sort_hashes(hash_paths: &mut Vec<(u64, PathBuf)>) {
    hash_paths.sort_by_key(|(hash, _)| *hash);
}

/// Opens an image file and decodes it.
///
/// # Arguments
///
/// * `file_path` - The path to the image file.
///
/// # Returns
///
/// * A `DynamicImage` if the file is successfully opened and decoded.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or decoded.
#[inline]
pub fn open_image(file_path: &PathBuf) -> Result<DynamicImage> {
    ImageReader::open(file_path)
        .map_err(|e| anyhow!("Error opening image {}: {}", file_path.display(), e))?
        .decode()
        .map_err(|e| anyhow!("Error decoding image {}: {}", file_path.display(), e))
}

/// Identifies duplicate images based on hash values.
///
/// # Arguments
///
/// * `hash_paths` - A slice of hash-path tuples.
/// * `remove` - A boolean indicating whether to delete duplicate files.
///
/// # Returns
///
/// * A hashmap mapping hash values to lists of duplicate file paths.
///
/// # Errors
///
/// Returns an error if a file fails to be removed when `remove` is set to `true`.
pub fn find_duplicates(
    hash_paths: &[(u64, PathBuf)],
    remove: bool,
) -> Result<HashMap<u64, Vec<PathBuf>>, Error> {
    let mut duplicates_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    for window in hash_paths.windows(2) {
        if let [(hash1, path1), (hash2, path2)] = window {
            if hash1 == hash2 {
                duplicates_map
                    .entry(*hash1)
                    .or_insert_with(Vec::new)
                    .extend(vec![path1.clone(), path2.clone()]);
            }
        }
    }

    if remove {
        for paths in duplicates_map.values() {
            for path in paths.iter().skip(1) {
                if let Err(e) = fs::remove_file(path) {
                    eprintln!("Failed to remove file {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(duplicates_map)
}

