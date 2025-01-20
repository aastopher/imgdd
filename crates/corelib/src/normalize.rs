use anyhow::Result;
use image::{DynamicImage, imageops::FilterType};


/// Normalizes an image by resizing it to a fixed 9x8 resolution and converting it to grayscale.
///
/// This function is used to prepare images for hashing by ensuring consistent dimensions and grayscale conversion.
///
/// # Arguments
///
/// * `image` - A reference to a `DynamicImage` to be normalized.
/// * `filter` - The down sampling method to use during resizing. **Options:**
///   - `FilterType::Nearest`
///   - `FilterType::Triangle`
///   - `FilterType::CatmullRom`
///   - `FilterType::Gaussian`
///   - `FilterType::Lanczos3`
///
/// # Returns
///
/// * A `DynamicImage` that has been resized to 9x8 and converted to grayscale.
#[inline]
pub fn proc(image: &DynamicImage, filter: FilterType) -> Result<DynamicImage> {
    Ok(image.resize_exact(9, 8, filter).grayscale())
}
