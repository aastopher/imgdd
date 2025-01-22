use anyhow::Result;
use image::{DynamicImage, imageops::FilterType};

/// Normalizes an image by resizing it to a given resolution and converting it to grayscale.
///
/// # Arguments
/// * `image` - A reference to a `DynamicImage` to be normalized.
/// * `filter` - The down sampling method to use during resizing. 
///     - **Options:** [`Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`]
/// * `size` - A tuple `(width, height)`.
///
/// # Returns
/// * A `DynamicImage` that has been resized to the given dimensions and converted to grayscale.
#[inline]
pub fn proc(image: &DynamicImage, filter: FilterType, size: (u32, u32)) -> Result<DynamicImage> {
    let (width, height) = size;
    Ok(image.resize_exact(width, height, filter).grayscale())
}
