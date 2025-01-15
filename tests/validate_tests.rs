#[cfg(test)]
mod tests {
    use imgdd::validate::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_validate_path() {
        let temp_dir = tempdir().unwrap();
        let binding = temp_dir.path().to_path_buf();
        let validated = validate_path(&binding).unwrap();
        assert_eq!(validated, temp_dir.path());

        let invalid_path = PathBuf::from("/non/existent/path");
        assert!(validate_path(&invalid_path).is_err());
    }
}
