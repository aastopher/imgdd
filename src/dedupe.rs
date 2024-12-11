use crate::image_hash::ImageHash;
use dashmap::DashMap;
use log::{debug, error, info};
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

// Use a lazy_static DashMap for caching distances
lazy_static::lazy_static! {
    static ref DISTANCE_CACHE: DashMap<(u64, u64), usize> = DashMap::new();
}

pub struct DedupeResult {
    pub unique_files: DashMap<ImageHash, PathBuf>,
    pub duplicates: HashSet<PathBuf>,
}

/// Calculate and cache Hamming distances
fn compute_distance(hash1: &ImageHash, hash2: &ImageHash) -> usize {
    let key = if hash1.get_hash() < hash2.get_hash() {
        (hash1.get_hash(), hash2.get_hash())
    } else {
        (hash2.get_hash(), hash1.get_hash())
    };

    // Use DashMap to check and store cached distances
    if let Some(distance) = DISTANCE_CACHE.get(&key) {
        return *distance;
    }

    let distance = hash1.hamming_distance(hash2);
    DISTANCE_CACHE.insert(key, distance);
    distance
}

/// Collect duplicates.
///
/// # Arguments
/// - `path`: The path to the directory containing images.
/// - `hamming_threshold`: The Hamming distance threshold for near-duplicates.
///
/// # Returns
/// A `DedupeResult` containing unique files and duplicates.
pub fn collect_duplicates(path: &PathBuf, hamming_threshold: usize) -> Result<DedupeResult> {
    let files: Vec<PathBuf> = fs::read_dir(path)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.is_file())
        .collect();

    let hashes: DashMap<ImageHash, PathBuf> = DashMap::new();
    let duplicates = DashMap::new();

    // Use Rayon to parallelize hashing
    files.par_iter().for_each(|file_path| {
        match image::open(file_path) {
            Ok(image) => match ImageHash::dhash(&image, 8) {
                Ok(hash) => {
                    let mut is_duplicate = false;

                    // Check for duplicates in existing hashes
                    hashes.iter().for_each(|existing| {
                        if compute_distance(&hash, existing.key()) <= hamming_threshold {
                            debug!(
                                "Near-duplicate found: {} is close to {}",
                                file_path.display(),
                                existing.value().display()
                            );
                            duplicates.insert(file_path.clone(), true);
                            is_duplicate = true;
                        }
                    });

                    // If no duplicates, add to unique hashes
                    if !is_duplicate {
                        hashes.insert(hash, file_path.clone());
                    }
                }
                Err(e) => error!("Failed to compute hash for {}: {}", file_path.display(), e),
            },
            Err(e) => error!("Failed to open image {}: {}", file_path.display(), e),
        }
    });

    Ok(DedupeResult {
        unique_files: hashes,
        duplicates: duplicates.into_iter().map(|(k, _)| k).collect(),
    })
}

/// Remove duplicate files.
pub fn remove_duplicates(duplicates: &HashSet<PathBuf>) -> Result<()> {
    duplicates.par_iter().for_each(|duplicate| {
        if let Err(e) = fs::remove_file(duplicate) {
            error!("Failed to remove file {}: {}", duplicate.display(), e);
        } else {
            info!("Removed duplicate file: {}", duplicate.display());
        }
    });
    Ok(())
}
