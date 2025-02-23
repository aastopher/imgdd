use anyhow::{anyhow, Result};
use std::path::PathBuf;
use image::{imageops::FilterType, DynamicImage, ImageReader};

/// Converts a string to a `FilterType`.
///
/// # Arguments
///
/// - `filter` - String specifying the filter type.
///     - **Options:** [`Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`]
///
/// # Returns
///
/// - A `FilterType` enum corresponding to the input string.
#[inline]
pub fn select_filter_type(filter: Option<&str>) -> FilterType {
    match filter.unwrap_or("nearest") {
        f if f.eq_ignore_ascii_case("nearest") => FilterType::Nearest,
        f if f.eq_ignore_ascii_case("triangle") => FilterType::Triangle,
        f if f.eq_ignore_ascii_case("catmullrom") => FilterType::CatmullRom,
        f if f.eq_ignore_ascii_case("gaussian") => FilterType::Gaussian,
        f if f.eq_ignore_ascii_case("lanczos3") => FilterType::Lanczos3,
        other => panic!("Unsupported filter type: {}", other),
    }
}

/// Selects a hashing algorithm.
///
/// # Arguments
///
/// - `algo` - String specifying the hashing algorithm.
///     - **Options:** [`aHash`, `mHash`, `dHash`, `pHash`, `wHash`]
///
/// # Returns
///
/// - A standardized `&'static str` representing the selected algorithm.
#[inline]
pub fn select_algo(algo: Option<&str>) -> &'static str {
    match algo.unwrap_or("dhash") {
        input if input.eq_ignore_ascii_case("dhash") => "dhash",
        input if input.eq_ignore_ascii_case("ahash") => "ahash",
        input if input.eq_ignore_ascii_case("mhash") => "mhash",
        input if input.eq_ignore_ascii_case("phash") => "phash",
        input if input.eq_ignore_ascii_case("whash") => "whash",
        other => panic!("Unsupported algorithm: {}", other),
    }
}


/// Validates a given path to ensure it exists and is a directory.
///
/// This function checks whether the provided path exists and is a directory,
/// returning an error if either condition is not met.
///
/// # Arguments
///
/// * `path` - A reference to a `PathBuf` representing the path to validate.
///
/// # Returns
///
/// * `Ok(&PathBuf)` - A reference to the validated `PathBuf` if the path exists and is a directory.
/// * `Err(anyhow::Error)` - An error indicating why the path is invalid.
#[inline]
pub fn validate_path(path: &PathBuf) -> Result<&PathBuf> {
    if !path.exists() {
        let message = format!("Path does not exist: {}", path.display());
        return Err(anyhow!(message));
    }

    if !path.is_dir() {
        let message = format!("Path is not a directory: {}", path.display());
        return Err(anyhow!(message));
    }

    Ok(path)
}


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
pub fn normalize(
    image: &DynamicImage,
    filter: FilterType,
    width: u32,
    height: u32,
) -> Result<DynamicImage> {
    Ok(image.resize_exact(width, height, filter).grayscale())
}


/// Opens an image file and decodes it.
///
/// # Arguments
///
/// * `file_path` - The path to the image file.
///
/// # Returns
///
/// * A `DynamicImage` if the file is successfully opened and decoded.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or decoded.
#[inline]
pub fn open_image(file_path: &PathBuf) -> Result<DynamicImage> {
    ImageReader::open(file_path)
        .map_err(|e| anyhow!("Error opening image {}: {}", file_path.display(), e))?
        .decode()
        .map_err(|e| anyhow!("Error decoding image {}: {}", file_path.display(), e))
}