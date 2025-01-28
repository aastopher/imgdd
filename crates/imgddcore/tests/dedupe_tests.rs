#[cfg(test)]
mod tests {
    use imgddcore::dedupe::*;
    use image::imageops::FilterType;
    use image::{DynamicImage, Rgba};
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::Write;
    use std::panic;

    fn create_mock_image() -> DynamicImage {
        DynamicImage::ImageRgba8(image::ImageBuffer::from_pixel(9, 8, Rgba([255, 0, 0, 255])))
    }

    #[test]
    fn test_collect_hashes() {
        let temp_dir = tempfile::tempdir().unwrap();
        let image_path = temp_dir.path().join("test_image.png");
        create_mock_image().save(&image_path).unwrap();
    
        let algorithms = ["dhash", "ahash", "mhash", "phash", "whash"];
        for algo in algorithms {
            let hashes = collect_hashes(&temp_dir.path().to_path_buf(), FilterType::Nearest, algo)
                .unwrap();
            assert_eq!(hashes.len(), 1, "Algorithm {} failed", algo);
        }
    }
    

    #[test]
    fn test_sort_hashes() {
        let mut hashes = vec![(2, PathBuf::from("b")), (1, PathBuf::from("a"))];
        sort_hashes(&mut hashes);
        assert_eq!(hashes, vec![(1, PathBuf::from("a")), (2, PathBuf::from("b"))]);
    }

    #[test]
    fn test_collect_hashes_unsupported_algo() {
        let temp_dir = tempfile::tempdir().unwrap();
        let image_path = temp_dir.path().join("test_image.png");
        create_mock_image().save(&image_path).unwrap();

        let result = panic::catch_unwind(|| {
            collect_hashes(&temp_dir.path().to_path_buf(), FilterType::Nearest, "unsupported_algo")
        });

        assert!(result.is_err()); // Should panic due to unsupported algorithm
    }

    #[test]
    fn test_collect_hashes_open_image_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let invalid_image_path = temp_dir.path().join("invalid_image.jpg");

        // Mock invalid image
        let mut file = File::create(&invalid_image_path).unwrap();
        file.write_all(b"not a valid image").unwrap();

        let result = collect_hashes(&temp_dir.path().to_path_buf(), FilterType::Nearest, "dhash");
        assert!(result.is_ok()); // Valid path, but should log errors for invalid image
    }

    #[test]
    fn test_collect_hashes_decode_image_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let invalid_image_path = temp_dir.path().join("test_image.jpg");

        // Create empty file that can't be decoded
        File::create(&invalid_image_path).unwrap();

        let result = collect_hashes(&temp_dir.path().to_path_buf(), FilterType::Nearest, "dhash");
        assert!(result.is_ok()); // Valid path, but decode errors should be logged
    }

    #[test]
    fn test_open_image_error_handling() {
        let temp_dir = tempfile::tempdir().unwrap();
        let invalid_image_path = temp_dir.path().join("nonexistent_image.jpg");

        // Assert file does not exist
        assert!(!invalid_image_path.exists());

        // Attempt to open non-existent image to trigger error
        let result = open_image(&invalid_image_path);
        assert!(result.is_err());

        if let Err(err) = result {
            let error_message = format!("Error opening image {}", invalid_image_path.display());
            assert!(err.to_string().contains(&error_message));
        }
    }

    #[test]
    fn test_find_duplicates_remove_file_success() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path_1 = temp_dir.path().join("test_file_1.txt");
        let file_path_2 = temp_dir.path().join("test_file_2.txt");
    
        // Create two dummy files
        std::fs::write(&file_path_1, b"file 1 content").unwrap();
        std::fs::write(&file_path_2, b"file 2 content").unwrap();
    
        // Mock duplicate hash paths
        let hash_paths = vec![
            (1, file_path_1.clone()),
            (1, file_path_2.clone()),
        ];
    
        // Test with `remove = true` to trigger file deletion
        let result = find_duplicates(&hash_paths, true);
        assert!(result.is_ok());
    
        // First file should remain
        assert!(file_path_1.exists());
    
        // Duplicate should be removed
        assert!(!file_path_2.exists());
    }

    #[test]
    fn test_find_duplicates_remove_file_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path_1 = temp_dir.path().join("test_file_1.txt");
        let file_path_2 = temp_dir.path().join("nonexistent_file.txt");
    
        // Create single dummy file
        std::fs::write(&file_path_1, b"file 1 content").unwrap();
    
        // Expect first file exists before test starts
        assert!(file_path_1.exists());
    
        // Mock duplicate hash paths, including a non-existent file
        let hash_paths = vec![
            (1, file_path_1.clone()),
            (1, file_path_2.clone()),
        ];
    
        // Test with `remove = true` to trigger file deletion
        let result = find_duplicates(&hash_paths, true);
        assert!(result.is_ok());
    
        // First file should remain untouched
        assert!(file_path_1.exists());
    
        // Second file should not exist, and removal should fail gracefully
        assert!(!file_path_2.exists(), "File {} should not exist.", file_path_2.display());
    }
    
}
