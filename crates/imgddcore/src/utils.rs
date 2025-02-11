use image::imageops::FilterType;

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
