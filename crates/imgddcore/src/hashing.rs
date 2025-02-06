use anyhow::Result;
use image::{DynamicImage, GenericImageView};

use dwt::wavelet::Haar;
use dwt::{Operation, Transform};
use rustdct::DctPlanner;

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
        let mut sum = 0u64;
        let mut pixels = [0u8; 64];

        // Collect pixel values and compute sum
        for (i, (_, _, pixel)) in image.pixels().enumerate().take(64) {
            pixels[i] = pixel[0]; // Grayscale value
            sum += pixels[i] as u64;
        }

        // Collect average pixel value
        let avg = sum / 64;

        // Compute hash and store bits in the correct order
        let mut hash = 0u64;
        for (i, &pixel) in pixels.iter().enumerate() {
            if pixel as u64 > avg {
                hash |= 1 << (63 - i); // reverse order
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
        let mut pixels = [0u8; 64];

        // Collect 64 pixel values
        for (i, pixel) in image.pixels().map(|p| p.2[0]).take(64).enumerate() {
            pixels[i] = pixel;
        }

        // Copy pixels so we don't modify the original array
        let mut pixels_copy = pixels;

        // Find median O(n)
        let mid = 32;
        let (low, median, _high) = pixels_copy.select_nth_unstable(mid);
        let median = (*median as u64 + low[mid - 1] as u64) / 2; // Compute true median

        // Compute hash
        let mut hash = 0u64;
        for (i, &pixel) in pixels.iter().enumerate() {
            if pixel as u64 > median {
                hash |= 1 << (63 - i); // reverse order
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
            let mut current = image.get_pixel(0, y)[0];
            for x in 1..9 {
                let next = image.get_pixel(x, y)[0];
                hash = (hash << 1) | (next > current) as u64;
                current = next;
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
        let mut pixels: Vec<f32> = image.pixels().map(|p| p.2[0] as f32).collect();

        // Plan DCT once for both rows and columns
        let mut planner = DctPlanner::new();
        let dct = planner.plan_dct2(IMG_SIZE);

        // Apply DCT row-wise in-place
        for row in pixels.chunks_exact_mut(IMG_SIZE) {
            dct.process_dct2(row);
        }

        // Apply DCT column-wise in-place
        for col in 0..IMG_SIZE {
            let mut col_values: [f32; IMG_SIZE] = [0.0; IMG_SIZE];

            for row in 0..IMG_SIZE {
                col_values[row] = pixels[row * IMG_SIZE + col];
            }

            dct.process_dct2(&mut col_values);

            for row in 0..IMG_SIZE {
                pixels[row * IMG_SIZE + col] = col_values[row];
            }
        }

        // Extract top-left 8x8 DCT coefficients (low frequencies)
        let mut dct_lowfreq = [0f32; HASH_SIZE * HASH_SIZE];
        for y in 0..HASH_SIZE {
            for x in 0..HASH_SIZE {
                dct_lowfreq[y * HASH_SIZE + x] = pixels[y * IMG_SIZE + x];
            }
        }

        // Compute median excluding DC coefficient
        let mut ac_coeffs = dct_lowfreq[1..].to_vec();
        let mid = ac_coeffs.len() / 2;
        ac_coeffs.select_nth_unstable_by(mid, |a, b| a.partial_cmp(b).unwrap());
        let median = ac_coeffs[mid];

        // Generate hash
        let mut hash = 0u64;
        for (i, &val) in dct_lowfreq.iter().enumerate() {
            if val > median {
                hash |= 1 << (63 - i);
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
        const HASH_SIZE: u32 = 8;
        let ll_max_level: usize = 3;

        // Allocate flat vector of normalized pixels (row–major order).
        let total_pixels = (HASH_SIZE * HASH_SIZE) as usize;
        let mut pixels = Vec::with_capacity(total_pixels);
        for y in 0..HASH_SIZE {
            for x in 0..HASH_SIZE {
                let pixel = image.get_pixel(x, y);
                pixels.push(pixel[0] as f32 / 255.0);
            }
        }

        // ---------- Remove low-level frequency (DC) component ---------- //
        // Perform a full forward Haar transform - 8×8 image (3 levels).
        pixels.transform(Operation::Forward, &Haar::new(), ll_max_level);

        // Zero out the DC coefficient.
        pixels[0] = 0.0;

        // Perform inverse Haar transform (reconstruct image).
        pixels.transform(Operation::Inverse, &Haar::new(), ll_max_level);

        // ---------- Compute median O(n) ---------- //
        let mid: usize = 32;
        // Clone flat pixel vector.
        let mut flat = pixels.clone();
        // Quicksort vector.
        flat.select_nth_unstable_by(mid, |a, b| a.partial_cmp(b).unwrap());
        // Compute median.
        let median = (flat[mid - 1] + flat[mid]) / 2.0;

        // Generate hash.
        let mut hash = 0u64;
        for (i, &val) in pixels.iter().enumerate() {
            if val > median {
                hash |= 1 << (63 - i);
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
