use anyhow::Result;
use image::{DynamicImage, imageops::FilterType};

pub fn proc(image: &DynamicImage, filter: FilterType) -> Result<DynamicImage> {
    Ok(image.resize_exact(8, 8, filter).grayscale())
}
