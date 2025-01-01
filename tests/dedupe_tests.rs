use imgdd::dedupe::{collect_hashes, find_duplicates, find_duplicates_with_threshold};
use std::path::PathBuf;

#[test]
fn test_collect_hashes() {
    let path = PathBuf::from("./imgs/test/apple_pie");
    let hashes = collect_hashes(&path).expect("Failed to collect hashes");
    assert!(!hashes.is_empty(), "Hashes should not be empty");
}

#[test]
fn test_find_duplicates() {
    let hashes_with_paths = vec![
        (0b101010, PathBuf::from("img1.png")),
        (0b101010, PathBuf::from("img2.png")),
        (0b111000, PathBuf::from("img3.png")),
    ];
    let duplicates = find_duplicates(&hashes_with_paths);
    assert_eq!(duplicates.len(), 1, "Should find one duplicate");
}

#[test]
fn test_find_duplicates_with_threshold() {
    let hashes_with_paths = vec![
        (0b101010, PathBuf::from("img1.png")),
        (0b111000, PathBuf::from("img2.png")),
    ];
    let duplicates = find_duplicates_with_threshold(&hashes_with_paths, 3);
    assert_eq!(duplicates.len(), 1, "Should find one duplicate within threshold");
}
