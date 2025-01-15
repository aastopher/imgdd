#[cfg(test)]
mod tests {
    use imgdd::dedupe::*;
    use image::imageops::FilterType;
    use image::{DynamicImage, Rgba};
    use std::path::PathBuf;

    fn create_mock_image() -> DynamicImage {
        DynamicImage::ImageRgba8(image::ImageBuffer::from_pixel(9, 8, Rgba([255, 0, 0, 255])))
    }

    #[test]
    fn test_collect_hashes() {
        let temp_dir = tempfile::tempdir().unwrap();
        let image_path = temp_dir.path().join("test_image.png");
        create_mock_image().save(&image_path).unwrap();

        let hashes = collect_hashes(&temp_dir.path().to_path_buf(), FilterType::Nearest, "dhash")
            .unwrap();
        assert_eq!(hashes.len(), 1);
    }

    #[test]
    fn test_sort_hashes() {
        let mut hashes = vec![(2, PathBuf::from("b")), (1, PathBuf::from("a"))];
        sort_hashes(&mut hashes);
        assert_eq!(hashes, vec![(1, PathBuf::from("a")), (2, PathBuf::from("b"))]);
    }

    #[test]
    fn test_find_duplicates() {
        let hash_paths = vec![
            (1, PathBuf::from("a")),
            (1, PathBuf::from("b")),
            (2, PathBuf::from("c")),
        ];
        let duplicates = find_duplicates(&hash_paths, false).unwrap();
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[&1].len(), 2);
    }
}
