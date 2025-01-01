use imgdd::utils::validate_path;
use std::path::PathBuf;

#[test]
fn test_validate_path_valid() {
    let valid_path = PathBuf::from("./imgs/test/apple_pie");
    assert!(validate_path(Some(valid_path)).is_ok(), "Valid path should pass");
}

#[test]
fn test_validate_path_invalid() {
    let invalid_path = PathBuf::from("./non_existent_path");
    assert!(validate_path(Some(invalid_path)).is_err(), "Invalid path should fail");
}
