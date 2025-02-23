#[cfg(test)]
mod selection_tests {
    use image::imageops::FilterType;
    use imgddcore::utils::{select_algo, select_filter_type};

    #[test]
    fn test_select_filter_type() {
        assert_eq!(select_filter_type(Some("nearest")), FilterType::Nearest);
        assert_eq!(select_filter_type(Some("triangle")), FilterType::Triangle);
        assert_eq!(
            select_filter_type(Some("catmullrom")),
            FilterType::CatmullRom
        );
        assert_eq!(select_filter_type(Some("gaussian")), FilterType::Gaussian);
        assert_eq!(select_filter_type(Some("lanczos3")), FilterType::Lanczos3);

        let result = std::panic::catch_unwind(|| select_filter_type(Some("unsupported")));
        assert!(
            result.is_err(),
            "Expected panic for unsupported filter type"
        );
    }

    #[test]
    fn test_select_algo() {
        assert_eq!(select_algo(Some("dhash")), "dhash");
        assert_eq!(select_algo(Some("ahash")), "ahash");
        assert_eq!(select_algo(Some("mhash")), "mhash");
        assert_eq!(select_algo(Some("phash")), "phash");
        assert_eq!(select_algo(Some("whash")), "whash");

        let result = std::panic::catch_unwind(|| select_algo(Some("unsupported")));
        assert!(result.is_err(), "Expected panic for unsupported algorithm");
    }

}



#[cfg(test)]
mod validate_tests {
    use imgddcore::utils::validate_path;
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
            assert_eq!(err.to_string(), "Path does not exist: /non/existent/path");
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


#[cfg(test)]
mod normalize_tests {
    use image::imageops::FilterType;
    use image::{DynamicImage, Rgba};
    use imgddcore::utils::normalize;

    fn create_mock_image() -> DynamicImage {
        DynamicImage::ImageRgba8(image::ImageBuffer::from_pixel(
            16,
            16,
            Rgba([255, 0, 0, 255]),
        ))
    }

    #[test]
    fn test_normalization() {
        let image = create_mock_image();
        let normalized89 = normalize(&image, FilterType::Nearest, 9, 8).unwrap();
        assert_eq!(normalized89.width(), 9);
        assert_eq!(normalized89.height(), 8);

        let normalized88 = normalize(&image, FilterType::Nearest, 8, 8).unwrap();
        assert_eq!(normalized88.width(), 8);
        assert_eq!(normalized88.height(), 8);
    }
        
}