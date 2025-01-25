use image::{DynamicImage, GenericImageView};
use rustdct::DctPlanner;
use anyhow::Result;

/// A structure representing the hash of an image as u64.
///
/// The `ImageHash` structure is used to store and compare the hash of an image for deduplication purposes.
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ImageHash {
    hash: u64,
}

impl ImageHash {
    /// Computes the average hash (aHash) of a given image.
    ///
    /// # Arguments
    /// * `image` - A reference to a `DynamicImage` for which the hash is to be calculated.
    ///
    /// # Returns
    /// * An `ImageHash` instance containing the computed aHash value.
    ///
    /// # Details
    /// **aHash (Average Hash):**
    /// - Simple and fast to compute.
    /// - Based on average brightness, making it suitable for detecting overall image similarity.
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
    ///
    /// # Details
    /// **mHash (Median Hash):**
    /// - Similar to aHash but uses the median brightness for more robustness to lighting changes.
    /// - Suitable for images with varying brightness or exposure levels.
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
    /// * `image` - A reference to a `DynamicImage` for which the hash is to be calculated.
    ///
    /// # Returns
    /// * An `ImageHash` instance containing the computed dHash value.
    ///
    /// # Details
    /// **dHash (Difference Hash):**
    /// - Encodes relative changes between adjacent pixels.
    /// - Resistant to small transformations like cropping or rotation.
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
    /// # Arguments:
    /// * `image` - A reference to a `DynamicImage` for which the hash is to be calculated.
    ///
    /// # Returns:
    /// * An `ImageHash` instance containing the computed pHash value.
    ///
    /// # Details
    /// **pHash (Perceptual Hash):**
    /// - Analyzes the frequency domain using Discrete Cosine Transform (DCT).
    /// - Focuses on low-frequency components, which are less affected by resizing or compression.
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
    ///
    /// # Arguments
    /// * `image` - A reference to a `DynamicImage` for which the hash is to be calculated.
    ///
    /// # Returns
    /// * An `ImageHash` instance containing the computed wHash value.
    ///
    /// # Details
    /// **wHash (Wavelet Hash):**
    /// - Uses Haar wavelet transformations to capture image features.
    /// - Robust against scaling, rotation, and noise.
    #[inline]
    pub fn whash(image: &DynamicImage) -> Result<Self> {
        const HASH_SIZE: usize = 8; // Hash size (8x8)
        let image_scale = HASH_SIZE * 2; // Scaled for decomposition

        // Convert image pixels to a normalized f64 array
        let pixels: Vec<f64> = image
            .pixels()
            .map(|p| p.2[0] as f64 / 255.0) // Normalize pixel values to [0.0, 1.0]
            .collect();

        // Calculate maximum Haar decomposition level based on image scale
        let ll_max_level = (image_scale as f64).log2().floor() as usize;

        // Perform Haar wavelet decomposition up to max_level
        let mut coeffs = vec![pixels.clone()];
        let mut current = pixels.clone();
        for _ in 0..ll_max_level {
            let size = (current.len() as f64).sqrt() as usize; // Assume a square image
            let half_size = size / 2;

            let mut next = vec![0.0; current.len()];
            for y in 0..half_size {
                for x in 0..half_size {
                    let top_left = current[y * size + x];
                    let top_right = current[y * size + x + half_size];
                    let bottom_left = current[(y + half_size) * size + x];
                    let bottom_right = current[(y + half_size) * size + x + half_size];

                    let avg = (top_left + top_right + bottom_left + bottom_right) / 4.0; // LL
                    let hor_diff = (top_left + top_right - bottom_left - bottom_right) / 4.0; // HL
                    let ver_diff = (top_left - top_right + bottom_left - bottom_right) / 4.0; // LH
                    let diag_diff = (top_left - top_right - bottom_left + bottom_right) / 4.0; // HH

                    let idx = y * size + x;
                    next[idx] = avg; // Approximation coefficients (LL)
                    next[idx + half_size] = hor_diff; // Horizontal details (HL)
                    next[(y + half_size) * size + x] = ver_diff; // Vertical details (LH)
                    next[(y + half_size) * size + x + half_size] = diag_diff; // Diagonal details (HH)
                }
            }

            coeffs.push(next.clone());
            current = next;
        }

        // Zero out LL component at max level (making it less sensitive to large-scale image changes)
        coeffs.last_mut().unwrap().iter_mut().for_each(|v| *v = 0.0);

        // Use LL coefficients at appropriate level (based on HASH_SIZE)
        let dwt_level = ll_max_level - (HASH_SIZE as f64).log2().floor() as usize;
        let low_freq = coeffs[dwt_level]
            .iter()
            .cloned()
            .take(HASH_SIZE * HASH_SIZE)
            .collect::<Vec<f64>>();

        // Calculate median of coefficients
        let mut sorted_low_freq = low_freq.clone();
        sorted_low_freq.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        let median = sorted_low_freq[HASH_SIZE * HASH_SIZE / 2];

        // Generate hash
        let mut hash = 0u64;
        for (i, &val) in low_freq.iter().enumerate() {
            if val > median {
                hash |= 1 << i;
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
