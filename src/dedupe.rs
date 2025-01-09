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
use pyo3::PyErr;

/// Collect hashes for all image files recursively in a directory and sort them.
pub fn collect_hashes(
    path: &PathBuf,
    filter: FilterType,
    algo: &str,
) -> Result<Vec<(u64, PathBuf)>, PyErr> {
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
                    let normalized = normalize::proc(&image, filter).ok()?;
                    let hash = match algo {
                        "dhash" => ImageHash::dhash(&normalized).ok()?,
                        _ => panic!("Unsupported hashing algorithm: {}", algo),
                    };
                    Some((hash.get_hash(), file_path.clone()))
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

/// Sort hashes by their hash value
#[inline]
pub fn sort_hashes(hash_paths: &mut Vec<(u64, PathBuf)>) {
    hash_paths.sort_by_key(|(hash, _)| *hash);
}


/// Open an image file using `ImageReader` to support multiple formats.
#[inline]
pub fn open_image(file_path: &PathBuf) -> Result<DynamicImage> {
    ImageReader::open(file_path)
        .map_err(|e| anyhow!("Error opening image {}: {}", file_path.display(), e))?
        .decode()
        .map_err(|e| anyhow!("Error decoding image {}: {}", file_path.display(), e))
}


/// Identify exact duplicates by comparing sorted hashes.
pub fn find_duplicates(
    hash_paths: &[(u64, PathBuf)],
    remove: bool,
) -> Result<HashMap<String, Vec<PathBuf>>, PyErr> {
    let mut duplicates = HashMap::new();

    for window in hash_paths.windows(2) {
        if let [(hash1, path1), (hash2, path2)] = window {
            if hash1 == hash2 {
                let hash_binary = format!("{:b}", hash1);
                duplicates
                    .entry(hash_binary)
                    .or_insert_with(Vec::new)
                    .extend(vec![path1.clone(), path2.clone()]);
            }
        }
    }

    // If `remove` is true, delete duplicate files
    if remove {
        for paths in duplicates.values() {
            for path in paths.iter().skip(1) {
                if let Err(e) = fs::remove_file(path) {
                    eprintln!("Failed to remove file {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(duplicates)
}