use imgdd::dedupe::{collect_hashes, find_duplicates};
use imgdd::logger;
use clap::{arg, command, ArgAction, value_parser, ValueHint};
use log::{info, debug};
use anyhow::Result;
use std::path::PathBuf;
use imgdd::utils::validate_path;

fn main() -> Result<()> {
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
                -v --verbose ... "Enable verbose output for debugging. Use -vv for maximum verbosity."
            )
            .action(ArgAction::Count),
        )
        .get_matches();

    // Set verbosity level
    match matches.get_count("verbose") {
        0 => info!("Running in normal mode."),
        1 => debug!("Running in verbose mode."),
        _ => debug!("Running in maximum verbosity mode."),
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

    // Print hashes and associated paths
    println!("Hashes and Paths (sorted):");
    for (hash, path) in &hashes_with_paths {
        println!("Hash: {:032b}, Path: {}", hash, path.display());
    }

    // Identify duplicates
    let duplicates = find_duplicates(&hashes_with_paths);

    // Print duplicates
    if duplicates.is_empty() {
        println!("No duplicates found.");
    } else {
        println!("Duplicates found:");
        for (dup1, dup2) in duplicates {
            println!("Duplicate: {} <-> {}", dup1.display(), dup2.display());
        }
    }

    Ok(())
}
