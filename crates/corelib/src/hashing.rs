use image::{DynamicImage, GenericImageView};
use rustdct::DctPlanner;
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
    /// Computes the average hash (aHash) of a given image.
    ///
    /// # Arguments
    ///
    /// * `image` - A reference to a `DynamicImage` for which the hash is to be calculated.
    ///
    /// # Returns
    ///
    /// * An `ImageHash` instance containing the computed aHash value.
    #[inline]
    pub fn ahash(image: &DynamicImage) -> Result<Self> {
        // Collect pixel values from normalized 8x8 image
        let pixels: Vec<u64> = image.pixels().map(|p| p.2[0] as u64).collect();

        // Calculate average pixel value
        let avg: u64 = pixels.iter().sum::<u64>() / pixels.len() as u64;

        // Compute hash by comparing each pixel to the average
        let mut hash = 0u64;
        for (i, &pixel) in pixels.iter().enumerate().take(64) {
            if pixel > avg {
                hash |= 1 << i;
            }
        }

        Ok(Self { hash })
    }


    /// Computes the median hash (mHash) of a given image.
    ///
    /// # Arguments
    /// * `image` - A reference to a `DynamicImage` for which the hash is to be calculated.
    ///
    /// # Returns
    /// * An `ImageHash` instance containing the computed mHash value.
    #[inline]
    pub fn mhash(image: &DynamicImage) -> Result<Self> {
        // Collect pixel values from normalized 8x8 image
        let pixels: Vec<u64> = image.pixels().map(|p| p.2[0] as u64).collect();
        
        // Calculate median for 64 pixels
        let mut sorted_pixels = pixels.clone();
        sorted_pixels.sort_unstable();
        let median = (sorted_pixels[31] + sorted_pixels[32]) / 2;
    
        // Compute hash by comparing each pixel to the median
        let mut hash = 0u64;
        for (i, &pixel) in pixels.iter().enumerate().take(64) {
            if pixel > median {
                hash |= 1 << i;
            }
        }
    
        Ok(Self { hash })
    }
    

    /// Computes the difference hash (dHash) of a given image.
    ///
    /// # Arguments
    ///
    /// * `image` - A reference to a `DynamicImage` for which the hash is to be calculated.
    ///
    /// # Returns
    ///
    /// * An `ImageHash` instance containing the computed dHash value.
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

    /// Computes the perceptual hash (pHash) of a given image.
    ///
    /// pHash analyzes the frequency domain of the image using a Discrete Cosine Transform (DCT). 
    /// It extracts low-frequency components, which are less susceptible to changes like resizing or compression, 
    /// making it ideal for perceptual similarity comparisons.
    ///
    /// # Arguments:
    /// * `image` - A reference to a `DynamicImage` for which the hash is to be calculated.
    ///
    /// # Returns:
    /// * An `ImageHash` instance containing the computed pHash value.
    #[inline]
    pub fn phash(image: &DynamicImage) -> Result<Self> {
        const IMG_SIZE: usize = 32;
        const HASH_SIZE: usize = 8;

        // Collect pixel values from normalized 32x32 grayscale image
        let mut pixels: Vec<f32> = image
            .pixels()
            .map(|p| p.2[0] as f32)
            .collect();

        // Plan DCT once for both rows and columns
        let mut planner = DctPlanner::new();
        let dct = planner.plan_dct2(IMG_SIZE);

        // Apply DCT row-wise in-place
        for row in pixels.chunks_exact_mut(IMG_SIZE) {
            dct.process_dct2(row);
        }

        // Temp buffer for column processing
        let mut col_buffer = vec![0f32; IMG_SIZE];

        // Apply DCT column-wise in-place
        for col in 0..IMG_SIZE {
            // Extract column into buffer
            for row in 0..IMG_SIZE {
                col_buffer[row] = pixels[row * IMG_SIZE + col];
            }
            // Perform DCT on the column
            dct.process_dct2(&mut col_buffer);
            // Store result back into the original pixel array
            for row in 0..IMG_SIZE {
                pixels[row * IMG_SIZE + col] = col_buffer[row];
            }
        }

        // Extract top-left 8x8 DCT coefficients (low frequencies)
        let mut dct_lowfreq = [0f32; HASH_SIZE * HASH_SIZE];
        for y in 0..HASH_SIZE {
            for x in 0..HASH_SIZE {
                dct_lowfreq[y * HASH_SIZE + x] = pixels[y * IMG_SIZE + x];
            }
        }

        // Sort the DCT coefficients (in-place to avoid unnecessary allocations)
        let mut sorted = dct_lowfreq;
        sorted.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        // Calculate the median from the sorted values
        let median_index = HASH_SIZE * HASH_SIZE / 2;
        let median = (sorted[median_index - 1] + sorted[median_index]) / 2.0;

        // Generate hash
        let mut hash = 0u64;
        for (i, &val) in dct_lowfreq.iter().enumerate() {
            if val > median {
                hash |= 1 << i;
            }
        }

        Ok(Self { hash })
    }


    /// Computes the wavelet hash (wHash) of a given image.
    #[inline]
    pub fn whash(_image: &DynamicImage) -> Result<Self> {
        // Wavelet hash implementation here
        Ok(Self { hash: 0 }) // Placeholder
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
