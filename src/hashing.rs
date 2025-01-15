use image::{DynamicImage, GenericImageView};
use anyhow::Result;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ImageHash {
    hash: u64,
}

impl ImageHash {
    #[inline]
    pub fn dhash(image: &DynamicImage) -> Result<Self> {
        let mut hash = 0u64;
        for y in 0..8 {
            for x in 0..8 {
                let current = image.get_pixel(x, y)[0];
                let next = image.get_pixel(x + 1, y)[0];
                hash = (hash << 1) | ((current > next) as u64);
            }
        }
        Ok(Self { hash })
    }

    #[inline]
    pub fn get_hash(&self) -> u64 {
        self.hash
    }
}
