use anyhow::Result;
use image::{DynamicImage, imageops::FilterType};

/// Normalizes an image by resizing it to a given resolution and converting it to grayscale.
///
/// # Arguments
/// * `image` - A reference to a `DynamicImage` to be normalized.
/// * `filter` - The down sampling method to use during resizing. 
///     - **Options:** [`Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`]
/// * `width` - The desired width of the resized image.
/// * `height` - The desired height of the resized image.
///
/// # Returns
/// * A `DynamicImage` that has been resized to the given dimensions and converted to grayscale.
#[inline]
pub fn proc(image: &DynamicImage, filter: FilterType, width: u32, height: u32) -> Result<DynamicImage> {
    Ok(image.resize_exact(width, height, filter).grayscale())
}
