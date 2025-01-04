use crate::hashing::ImageHash;
use crate::normalize;
use anyhow::Result;
use image::imageops::FilterType;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn collect_dupes(
    path: &PathBuf,
    filter: FilterType,
    algo: &str,
    remove: bool,
) -> Result<HashMap<u64, Vec<PathBuf>>> {
    // Collect image files recursively
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
                Err(_) => None,
            }
        })
        .collect();

    // Sort by hash for grouping duplicates
    hashes_with_paths.sort_by_key(|(hash, _)| *hash);

    // Group duplicates by hash
    let mut duplicates = HashMap::new();
    for (hash, path) in hashes_with_paths {
        duplicates.entry(hash).or_insert_with(Vec::new).push(path);
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
