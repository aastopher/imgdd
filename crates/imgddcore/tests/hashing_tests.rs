#[cfg(test)]
mod tests {
    use imgddcore::hashing::ImageHash;
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

    #[test]
    fn test_ahash() -> Result<()> {
        let test_image = create_mock_image((8, 8));
        let hash = ImageHash::ahash(&test_image)?;
        println!("aHash: {:064b}", hash.get_hash());
        let expected_hash = 0b0101010101010101010101010101010101010101010101010101010101010101;
        assert_eq!(hash.get_hash(), expected_hash, "aHash does not match expected value");

        Ok(())
    }

    #[test]
    fn test_mhash() -> Result<()> {
        let test_image = create_mock_image((8, 8));
        let hash = ImageHash::mhash(&test_image)?;
        println!("mHash: {:064b}", hash.get_hash());
        let expected_hash = 0b0101010101010101010101010101010101010101010101010101010101010101;
        
        assert_eq!(hash.get_hash(), expected_hash, "mHash does not match expected value");

        Ok(())
    }

    #[test]
    fn test_dhash() -> Result<()> {
        let test_image = create_mock_image((9, 8));
        let hash = ImageHash::dhash(&test_image)?;
        println!("dHash: {:064b}", hash.get_hash());
        let expected_hash = 0b1010101010101010101010101010101010101010101010101010101010101010;
        assert_eq!(hash.get_hash(), expected_hash, "dHash does not match expected value");

        Ok(())
    }

    #[test]
    fn test_phash() -> Result<()> {
        let test_image = create_mock_image((32, 32));
        let hash = ImageHash::phash(&test_image)?;
        let expected_hash = 0b0000000000000000000000000000000000000000000000000000000010101011;
        println!("pHash: {:064b}", hash.get_hash());
        assert_eq!(hash.get_hash(), expected_hash, "pHash does not match expected value");

        Ok(())
    }

    #[test]
    fn test_whash() -> Result<()> {
        let test_image = create_mock_image((8, 8));
        let hash = ImageHash::whash(&test_image)?;
        println!("wHash: {:064b}", hash.get_hash());
        let expected_hash = 0b0000000000000000000000000000000000000101000001010000010100000101;

        assert_eq!(hash.get_hash(), expected_hash, "wHash does not match expected value");

        Ok(())
    }

}