#[cfg(test)]
mod tests {
    use imgddcore::validate::*;
    use std::path::PathBuf;
    use tempfile::{tempdir, NamedTempFile};

    #[test]
    fn test_validate_path_exists() {
        let temp_dir = tempdir().unwrap();
        let binding = temp_dir.path().to_path_buf();
        let validated = validate_path(&binding).unwrap();
        assert_eq!(validated, temp_dir.path());
    }

    #[test]
    fn test_validate_path_does_not_exist() {
        let invalid_path = PathBuf::from("/non/existent/path");
        let result = validate_path(&invalid_path);
        assert!(result.is_err());

        if let Err(err) = result {
            assert_eq!(
                err.to_string(),
                "Path does not exist: /non/existent/path"
            );
        }
    }

    #[test]
    fn test_validate_path_not_directory() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_path_buf();
        let result = validate_path(&file_path);
        assert!(result.is_err());

        if let Err(err) = result {
            assert_eq!(
                err.to_string(),
                format!("Path is not a directory: {}", file_path.display())
            );
        }
    }
}
