use image::{DynamicImage, GenericImageView};
use anyhow::Result;

/// A structure representing the hash of an image.
///
/// The `ImageHash` structure is used to store and compare the hash of an image for deduplication purposes.
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ImageHash {
    /// Computed hash value.
    hash: u64,
}

impl ImageHash {
    /// Computes the difference hash (dHash) of a given image.
    ///
    /// # Arguments
    ///
    /// * `image` - A reference to a `DynamicImage` for which the hash is to be calculated.
    ///
    /// # Returns
    ///
    /// * An `ImageHash` instance containing the computed dHash value.
    ///
    /// # Errors
    ///
    /// Returns an error if the hash calculation fails for any reason.
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

    /// Retrieves the computed hash value.
    ///
    /// # Returns
    ///
    /// * Hash value as a `u64`.
    #[inline]
    pub fn get_hash(&self) -> u64 {
        self.hash
    }
}
