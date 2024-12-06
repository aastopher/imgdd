use anyhow::Result;
use image::{DynamicImage, ImageBuffer, Luma};
use ndarray::Array2;
// use log::{debug};

#[derive(Eq, PartialEq, Hash)]
pub struct ImageHash {
    hash: Array2<bool>,
}

impl ImageHash {
    /// Computes a difference hash (dhash) for the given image.
    ///
    /// # Arguments
    /// - `image`: A `DynamicImage` instance.
    /// - `hash_size`: The size of the hash grid (e.g., 8 for an 8x8 grid).
    ///
    /// # Returns
    /// A new `ImageHash` instance representing the computed hash.
    pub fn dhash(image: &DynamicImage, hash_size: u32) -> Result<Self> {
        // Remove alpha channel if present and convert to RGB
        let rgb_image = image.to_rgb8();
        // debug!("removed alpha channel");

        // Convert to grayscale
        let grayscale = DynamicImage::ImageRgb8(rgb_image).to_luma8();
        // debug!("converted to grayscale");

        // Resize the image using Lanczos3 filter
        let resized = image::imageops::resize(
            &grayscale,
            hash_size + 1,
            hash_size,
            image::imageops::FilterType::Lanczos3,
        );
        // debug!("image successfully resized");

        // Apply quantile normalization
        let normalized = Self::quantile_normalize(&resized)?;
        // debug!("quantile normalization applied");

        // Compute differences between adjacent pixels
        let mut hash = Array2::<bool>::default((hash_size as usize, hash_size as usize));
        for y in 0..hash_size {
            for x in 0..hash_size {
                hash[[y as usize, x as usize]] =
                    normalized.get_pixel(x, y)[0] > normalized.get_pixel(x + 1, y)[0];
            }
        }
        // debug!("image hash successfully computed");

        Ok(Self { hash })
    }

    /// Applies quantile normalization to the image.
    fn quantile_normalize(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>> {
        let mut pixel_values: Vec<u8> = image.pixels().map(|p| p[0]).collect();
        pixel_values.sort_unstable();

        let len = pixel_values.len();
        let quantiles: Vec<u8> = (0..len)
            .map(|i| ((i as f64 / (len - 1) as f64) * 255.0).round() as u8)
            .collect();

        let mut normalized_image = image.clone();
        for (_x, _y, pixel) in normalized_image.enumerate_pixels_mut() {
            let original_value = pixel[0];
            let rank = pixel_values.binary_search(&original_value).unwrap_or_else(|x| x);
            pixel[0] = quantiles[rank];
        }

        Ok(normalized_image)
    }

    /// Computes the Hamming distance between two image hashes.
    pub fn hamming_distance(&self, other: &Self) -> usize {
        self.hash
            .iter()
            .zip(other.hash.iter())
            .filter(|(a, b)| a != b)
            .count()
    }
}
