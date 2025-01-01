use crate::image_hash::ImageHash;
use rayon::prelude::*;
use walkdir::WalkDir;
use anyhow::Result;
use std::path::PathBuf;

/// Collect hashes for all image files recursively in a directory and sort them.
pub fn collect_hashes(path: &PathBuf) -> Result<Vec<(u64, PathBuf)>> {
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
                Ok(image) => match ImageHash::dhash(&image) {
                    Ok(hash) => Some((hash.get_hash(), file_path.clone())),
                    Err(e) => {
                        eprintln!("Failed to compute hash for {}: {}", file_path.display(), e);
                        None
                    }
                },
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
pub fn find_duplicates(hashes_with_paths: &[(u64, PathBuf)]) -> Vec<(PathBuf, PathBuf)> {
    let mut duplicates = Vec::new();

    for window in hashes_with_paths.windows(2) {
        if let [(hash1, path1), (hash2, path2)] = window {
            if hash1 == hash2 {
                duplicates.push((path1.clone(), path2.clone()));
            }
        }
    }

    duplicates
}

/// Identify duplicates within a Hamming distance threshold.
pub fn find_duplicates_with_threshold(
    hashes_with_paths: &[(u64, PathBuf)],
    threshold: u32,
) -> Vec<(PathBuf, PathBuf)> {
    let mut duplicates = Vec::new();

    for (i, (hash1, path1)) in hashes_with_paths.iter().enumerate() {
        for (hash2, path2) in hashes_with_paths.iter().skip(i + 1) {
            let hamming_distance = ImageHash { hash: *hash1 }
                .hamming_distance(&ImageHash { hash: *hash2 });

            if hamming_distance <= threshold as usize {
                duplicates.push((path1.clone(), path2.clone()));
            }
        }
    }

    duplicates
}
