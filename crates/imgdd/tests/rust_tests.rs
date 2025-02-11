#[cfg(test)]
mod tests {
    use imgdd::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_hash_with_valid_inputs() {
        let temp_dir = tempdir().unwrap();
        let image_path = temp_dir.path().join("test_image.png");

        // Create an invalid image
        let mut file = File::create(&image_path).unwrap();
        file.write_all(b"not a valid image").unwrap();

        let result = hash(
            temp_dir.path().to_path_buf(),
            Some("nearest"),
            Some("dhash"),
            Some(false),
        );
        assert!(result.is_ok(), "Hash function failed: {:?}", result.err());
    }

    #[test]
    fn test_hash_with_invalid_path() {
        let invalid_path = PathBuf::from("/non/existent/path");
        let result = hash(
            invalid_path.clone(),
            Some("nearest"),
            Some("dhash"),
            Some(false),
        );
        assert!(
            result.is_err(),
            "Expected error for invalid path: {:?}",
            invalid_path
        );
    }

    #[test]
    fn test_hash_with_sorting() {
        let img_dir = PathBuf::from("../../imgs/test/apple_pie");
        let result = hash(img_dir, Some("nearest"), Some("dhash"), Some(true));

        assert!(result.is_ok(), "Hash function failed: {:?}", result.err());

        // Unwrap result and assert expected number of hashes
        let hash_paths = result.unwrap();
        assert_eq!(
            hash_paths.len(),
            10,
            "Expected 10 hashes, got {}",
            hash_paths.len()
        );

        // Assert hashes sorted
        let sorted = hash_paths.windows(2).all(|w| w[0].0 <= w[1].0);
        assert!(sorted, "Hashes are not sorted: {:?}", hash_paths);
    }

    #[test]
    fn test_dupes_with_valid_inputs() {
        let temp_dir = tempdir().unwrap();
        let image_path_1 = temp_dir.path().join("test_image_1.png");
        let image_path_2 = temp_dir.path().join("test_image_2.png");

        // Create an invalid images
        let mut file1 = File::create(&image_path_1).unwrap();
        file1.write_all(b"not a valid image").unwrap();

        let mut file2 = File::create(&image_path_2).unwrap();
        file2.write_all(b"not a valid image").unwrap();

        let result = dupes(
            temp_dir.path().to_path_buf(),
            Some("nearest"),
            Some("dhash"),
            false,
        );
        assert!(result.is_ok(), "Dupes function failed: {:?}", result.err());

        let duplicates = result.unwrap();
        assert_eq!(
            duplicates.len(),
            0,
            "Expected no duplicates, but found some"
        );
    }

    #[test]
    fn test_dupes_with_invalid_path() {
        let invalid_path = PathBuf::from("/non/existent/path");
        let result = dupes(invalid_path.clone(), Some("nearest"), Some("dhash"), false);
        assert!(
            result.is_err(),
            "Expected error for invalid path: {:?}",
            invalid_path
        );
    }
}
