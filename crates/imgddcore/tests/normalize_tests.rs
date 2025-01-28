#[cfg(test)]
mod tests {
    use imgddcore::normalize::*;
    use image::imageops::FilterType;
    use image::{DynamicImage, Rgba};

    fn create_mock_image() -> DynamicImage {
        DynamicImage::ImageRgba8(image::ImageBuffer::from_pixel(16, 16, Rgba([255, 0, 0, 255])))
    }

    #[test]
    fn test_normalization() {
        let image = create_mock_image();
        let normalized89 = proc(&image, FilterType::Nearest, 9, 8).unwrap();
        assert_eq!(normalized89.width(), 9);
        assert_eq!(normalized89.height(), 8);

        let normalized88 = proc(&image, FilterType::Nearest, 8, 8).unwrap();
        assert_eq!(normalized88.width(), 8);
        assert_eq!(normalized88.height(), 8);
    }
}
