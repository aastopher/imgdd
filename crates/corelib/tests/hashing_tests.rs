#[cfg(test)]
mod tests {
    use corelib::hashing::ImageHash;
    use image::{DynamicImage, Rgba};
    use anyhow::Result;

    /// Creates a mock image with alternating pixel values for testing.
    fn create_mock_image(size: (u32, u32)) -> DynamicImage {
        let (width, height) = size;
        DynamicImage::ImageRgba8(image::ImageBuffer::from_fn(width, height, |x, _| {
            if x % 2 == 0 {
                Rgba([255, 0, 0, 255]) // Red pixel
            } else {
                Rgba([0, 0, 0, 255]) // Black pixel
            }
        }))
    }

    /// Tests the `aHash` implementation with a known mock image.
    #[test]
    fn test_ahash() -> Result<()> {
        let mock_image = create_mock_image((8, 8));
        let hash = ImageHash::ahash(&mock_image)?;
        let expected_hash = 0b0101010101010101010101010101010101010101010101010101010101010101;
        assert_eq!(hash.get_hash(), expected_hash, "aHash does not match expected value");

        Ok(())
    }

    /// Tests the `mHash` implementation with a known mock image.
    #[test]
    fn test_mhash() -> Result<()> {
        let mock_image = create_mock_image((8, 8));
        let hash = ImageHash::mhash(&mock_image)?;
        let expected_hash = 0b0101010101010101010101010101010101010101010101010101010101010101;
        
        assert_eq!(hash.get_hash(), expected_hash, "mHash does not match expected value");

        Ok(())
    }

    /// Tests the `dHash` implementation with a known mock image.
    #[test]
    fn test_dhash() -> Result<()> {
        let mock_image = create_mock_image((9, 8));
        let hash = ImageHash::dhash(&mock_image)?;
        let expected_hash = 0b1010101010101010101010101010101010101010101010101010101010101010;
        assert_eq!(hash.get_hash(), expected_hash, "dHash does not match expected value");

        Ok(())
    }


    /// Tests the `pHash` implementation with a known mock image.
    #[test]
    fn test_phash() -> Result<()> {
        let mock_image = create_mock_image((32, 32));
        let hash = ImageHash::phash(&mock_image)?;
        let expected_hash = 0b0000000000000000000000000000000000000000000000000000000010101011;
        
        assert_eq!(hash.get_hash(), expected_hash, "pHash does not match expected value");

        Ok(())
    }

}
