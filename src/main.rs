use imgdd::logger;
use imgdd::utils;
use imgdd::image_hash;

use clap::{arg, command, ArgAction, value_parser, ValueHint};
use log::{info, debug, error};
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::fs;
use utils::validate_path;
use image_hash::ImageHash;

fn main() -> Result<()> {
    logger::init();

    let matches = command!()
        .about("A CLI tool for visually deduping image directories")
        .version("1.0")
        .author("Aaron Stopher <aaron.stopher@gmail.com>")
        .arg(
            arg!(
                -p --path <PATH> "Specify a directory path. Defaults to the current directory."
            )
            .value_parser(value_parser!(PathBuf))
            .value_hint(ValueHint::DirPath)
            .required(false),
        )
        .arg(
            arg!(
                -d --distance <DISTANCE> "Set the Hamming distance threshold for duplicate detection."
            )
            .value_parser(value_parser!(usize))
            .default_value("12"),
        )
        .arg(
            arg!(
                -v --verbose ... "Enable verbose output for debugging. Use -vv for maximum verbosity."
            )
            .action(ArgAction::Count),
        )
        .get_matches();

    match matches.get_count("verbose") {
        0 => info!("Running in normal mode."),
        1 => debug!("Running in verbose mode."),
        _ => debug!("Running in maximum verbosity mode."),
    }

    // Validate path or use the current directory
    let path = validate_path(matches.get_one::<PathBuf>("path").cloned())?;
    info!("Using path: {}", path.display());

    // Retrieve Hamming distance threshold
    let hamming_threshold = *matches.get_one::<usize>("distance").unwrap_or(&12);
    info!("Using Hamming distance threshold: {}", hamming_threshold);

    let mut hash_to_path: HashMap<ImageHash, PathBuf> = HashMap::new();
    let mut duplicates: HashSet<PathBuf> = HashSet::new();

    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() {
            match image::open(&file_path) {
                Ok(image) => match ImageHash::dhash(&image, 8) {
                    Ok(hash) => {
                        let mut is_duplicate = false;
                        for (existing_hash, existing_path) in &hash_to_path {
                            if hash.hamming_distance(existing_hash) <= hamming_threshold {
                                debug!(
                                    "Near-duplicate found: {} is close to {}",
                                    file_path.display(),
                                    existing_path.display()
                                );
                                duplicates.insert(file_path.clone());
                                is_duplicate = true;
                                break;
                            }
                        }

                        if !is_duplicate {
                            hash_to_path.insert(hash, file_path.clone());
                        }
                    }
                    Err(e) => {
                        error!("Failed to compute hash for {}: {}", file_path.display(), e);
                    }
                },
                Err(e) => {
                    error!("Failed to open image {}: {}", file_path.display(), e);
                }
            }
        }
    }

    // Remove duplicate files
    for duplicate in duplicates {
        match fs::remove_file(&duplicate) {
            Ok(_) => info!("Removed duplicate file: {}", duplicate.display()),
            Err(e) => error!("Failed to remove file {}: {}", duplicate.display(), e),
        }
    }

    info!("Duplicate removal complete. Unique files kept: {}", hash_to_path.len());

    Ok(())
}
