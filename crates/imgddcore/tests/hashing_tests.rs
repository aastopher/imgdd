#[cfg(test)]
mod tests {
    use anyhow::Result;
    use image::{DynamicImage, Rgba};
    use imgddcore::hashing::ImageHash;

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
        .grayscale()
    }

    #[test]
    fn test_ahash() -> Result<()> {
        let test_image = create_mock_image((8, 8));
        let hash = ImageHash::ahash(&test_image)?;
        let expected_hash = 0b1010101010101010101010101010101010101010101010101010101010101010u64;
        let expected_bytes = expected_hash.to_be_bytes();
        assert_eq!(
            hash.get_hash(),
            expected_bytes.as_slice(),
            "aHash does not match expected value"
        );
        assert_eq!(hash.num_bytes(), 8, "aHash size should be 8");

        Ok(())
    }

    #[test]
    fn test_mhash() -> Result<()> {
        let test_image = create_mock_image((8, 8));
        let hash = ImageHash::mhash(&test_image)?;
        let expected_hash = 0b1010101010101010101010101010101010101010101010101010101010101010u64;
        let expected_bytes = expected_hash.to_be_bytes();
        assert_eq!(
            hash.get_hash(),
            expected_bytes.as_slice(),
            "mHash does not match expected value"
        );
        assert_eq!(hash.num_bytes(), 8, "mHash size should be 8");

        Ok(())
    }

    #[test]
    fn test_dhash() -> Result<()> {
        let test_image = create_mock_image((9, 8));
        let hash = ImageHash::dhash(&test_image)?;
        let expected_hash = 0b0101010101010101010101010101010101010101010101010101010101010101u64;
        let expected_bytes = expected_hash.to_be_bytes();
        assert_eq!(
            hash.get_hash(),
            expected_bytes.as_slice(),
            "dHash does not match expected value"
        );
        assert_eq!(hash.num_bytes(), 8, "dHash size should be 8");

        Ok(())
    }

    #[test]
    fn test_phash() -> Result<()> {
        let test_image = create_mock_image((32, 32));
        let hash = ImageHash::phash(&test_image, 8)?;
        let expected_hash = 0b1101010100000000000000000000000000000000000000000000000000000000u64;
        let expected_bytes = expected_hash.to_be_bytes();
        assert_eq!(
            hash.get_hash(),
            expected_bytes.as_slice(),
            "pHash does not match expected value"
        );
        assert_eq!(hash.num_bytes(), 8, "pHash size should be 8");

        Ok(())
    }

    #[test]
    fn test_phash_size_16() -> Result<()> {
        let test_image = create_mock_image((32, 32));
        let hash = ImageHash::phash(&test_image, 16)?;
        let expected_bytes: [u8; 32] = [
            0b11010101, 0b1010101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(
            hash.get_hash(),
            expected_bytes.as_slice(),
            "pHash does not match expected value"
        );
        assert_eq!(hash.num_bytes(), 32, "pHash size should be 32");

        Ok(())
    }

    #[test]
    fn test_whash() -> Result<()> {
        let test_image = create_mock_image((8, 8));
        let hash = ImageHash::whash(&test_image)?;
        let expected_hash = 0b1010101010101010101010101010101010101010101010101010101010101010u64;
        let expected_bytes = expected_hash.to_be_bytes();
        assert_eq!(
            hash.get_hash(),
            expected_bytes.as_slice(),
            "wHash does not match expected value"
        );
        assert_eq!(hash.num_bytes(), 8, "wHash size should be 8");

        Ok(())
    }

    #[test]
    fn test_hash_comparison() -> Result<()> {
        let test_image = create_mock_image((8, 8));
        let hash1 = ImageHash::ahash(&test_image)?;
        let hash2 = ImageHash::ahash(&test_image)?;
        assert_eq!(hash1, hash2, "Same image should produce same hash");

        Ok(())
    }
}
