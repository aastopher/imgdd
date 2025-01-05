use crate::hashing::ImageHash;
use crate::normalize;
use anyhow::Result;
use image::imageops::FilterType;
use rayon::prelude::*;
use walkdir::WalkDir;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Collect hashes for all image files recursively in a directory and sort them.
pub fn collect_hashes(
    path: &PathBuf,
    filter: FilterType,
    algo: &str,
) -> Result<Vec<(u64, PathBuf)>> {
    let files: Vec<PathBuf> = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().to_path_buf())
        .collect();

    let mut hashes_with_paths: Vec<(u64, PathBuf)> = files
        .par_iter()
        .filter_map(|file_path| {
            match image::open(file_path) {
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

    // Sort the hashes by their hash value
    hashes_with_paths.sort_by_key(|(hash, _)| *hash);

    Ok(hashes_with_paths)
}

/// Identify exact duplicates by comparing sorted hashes.
pub fn find_duplicates(
    hashes_with_paths: &[(u64, PathBuf)],
    remove: bool,
) -> Result<HashMap<String, Vec<PathBuf>>> {
    let mut duplicates = HashMap::new();

    for window in hashes_with_paths.windows(2) {
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
