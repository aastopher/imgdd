use imgdd::dedupe::{collect_duplicates, remove_duplicates};
use imgdd::logger;
use imgdd::utils;
use clap::{arg, command, ArgAction, value_parser, ValueHint};
use log::{info, debug};
use anyhow::Result;
use std::path::PathBuf;
use utils::validate_path;

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

    let path = validate_path(matches.get_one::<PathBuf>("path").cloned())?;
    let hamming_threshold = *matches.get_one::<usize>("distance").unwrap_or(&12);

    // Collect duplicates
    let dedupe_result = collect_duplicates(&path, hamming_threshold)?;
    info!(
        "Duplicate collection complete. Unique files: {}, Duplicates: {}",
        dedupe_result.unique_files.len(),
        dedupe_result.duplicates.len()
    );

    // Remove duplicates
    remove_duplicates(&dedupe_result.duplicates)?;

    info!("Duplicate removal complete.");
    Ok(())
}
