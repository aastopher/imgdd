#[cfg(test)]
mod tests {
    use imgdd::normalize::*;
    use image::imageops::FilterType;
    use image::{DynamicImage, Rgba};

    fn create_mock_image() -> DynamicImage {
        DynamicImage::ImageRgba8(image::ImageBuffer::from_pixel(16, 16, Rgba([255, 0, 0, 255])))
    }

    #[test]
    fn test_normalization() {
        let image = create_mock_image();
        let normalized = proc(&image, FilterType::Nearest).unwrap();
        assert_eq!(normalized.width(), 9);
        assert_eq!(normalized.height(), 8);
    }
}
