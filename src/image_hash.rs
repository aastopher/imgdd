use anyhow::Result;
use image::{DynamicImage};

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ImageHash {
    hash: u64,
}

impl ImageHash {
    pub fn dhash(image: &DynamicImage, hash_size: u32) -> Result<Self> {
        let grayscale = image.to_luma8();
        let resized = image::imageops::resize(
            &grayscale,
            hash_size + 1,
            hash_size,
            image::imageops::FilterType::Nearest,
        );

        let mut hash = 0u64;
        for y in 0..hash_size {
            for x in 0..hash_size {
                let current = resized.get_pixel(x, y)[0];
                let next = resized.get_pixel(x + 1, y)[0];
                hash = (hash << 1) | ((current > next) as u64);
            }
        }

        Ok(Self { hash })
    }

    pub fn hamming_distance(&self, other: &Self) -> usize {
        (self.hash ^ other.hash).count_ones() as usize
    }

    pub fn get_hash(&self) -> u64 {
        self.hash
    }
}
