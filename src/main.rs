use imgdd::dedupe::{collect_hashes, find_duplicates, find_duplicates_with_threshold};
use imgdd::logger;
use clap::{arg, command, ArgAction, value_parser, ValueHint};
use log::{info, debug, LevelFilter};
use anyhow::Result;
use std::path::PathBuf;
use imgdd::utils::validate_path;

fn main() -> Result<()> {
    // Initialize the logger with the default level
    logger::init();

    let matches = command!()
        .about("A CLI tool for visually deduping image directories")
        .version("1.0")
        .author("Aaron Stopher <aaron.stopher@gmail.com>")
        .arg(
            arg!(
                -p --path <PATH> "Specify the root directory path. Defaults to the current directory."
            )
            .value_parser(value_parser!(PathBuf))
            .value_hint(ValueHint::DirPath)
            .required(false),
        )
        .arg(
            arg!(
                -t --threshold <THRESHOLD> "Set the Hamming distance threshold for duplicate detection."
            )
            .value_parser(value_parser!(u32))
            .required(false),
        )
        .arg(
            arg!(
                -v --verbose ... "Enable verbose output for debugging. Use -vv for maximum verbosity."
            )
            .action(ArgAction::Count),
        )
        .get_matches();

    // Set verbosity level based on the number of -v flags
    match matches.get_count("verbose") {
        0 => log::set_max_level(LevelFilter::Info),  // Default level
        1 => log::set_max_level(LevelFilter::Debug), // One -v flag
        _ => log::set_max_level(LevelFilter::Trace), // Two or more -v flags
    }

    // Validate and resolve the root path
    let path = validate_path(Some(
        matches
            .get_one::<PathBuf>("path")
            .cloned()
            .unwrap_or_else(|| std::env::current_dir().expect("Failed to get the current directory")),
    ))?;

    // Collect hashes recursively
    let hashes_with_paths = collect_hashes(&path)?;

    // Log hashes and associated paths
    debug!("Hashes and Paths (sorted):");
    for (hash, path) in &hashes_with_paths {
        debug!("Hash: {:032b}, Path: {}", hash, path.display());
    }

    // Determine and process duplicates based on the threshold
    let duplicates = if let Some(threshold) = matches.get_one::<u32>("threshold").cloned() {
        info!("Using Hamming distance threshold: {}", threshold);
        find_duplicates_with_threshold(&hashes_with_paths, threshold)
    } else {
        info!("Performing exact duplicate detection.");
        find_duplicates(&hashes_with_paths)
    };

    // Log duplicates
    if duplicates.is_empty() {
        info!("No duplicates found.");
    } else {
        info!("Duplicates found:");
        for (dup1, dup2) in duplicates {
            info!("Duplicate: {} <-> {}", dup1.display(), dup2.display());
        }
    }

    Ok(())
}
