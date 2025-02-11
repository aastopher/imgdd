use image::imageops::FilterType;
use imgddcore::utils::{select_algo, select_filter_type};


#[test]
fn test_select_filter_type() {
    assert_eq!(select_filter_type(Some("nearest")), FilterType::Nearest);
    assert_eq!(select_filter_type(Some("triangle")), FilterType::Triangle);
    assert_eq!(
        select_filter_type(Some("catmullrom")),
        FilterType::CatmullRom
    );
    assert_eq!(select_filter_type(Some("gaussian")), FilterType::Gaussian);
    assert_eq!(select_filter_type(Some("lanczos3")), FilterType::Lanczos3);

    let result = std::panic::catch_unwind(|| select_filter_type(Some("unsupported")));
    assert!(
        result.is_err(),
        "Expected panic for unsupported filter type"
    );
}

#[test]
fn test_select_algo() {
    assert_eq!(select_algo(Some("dhash")), "dhash");
    assert_eq!(select_algo(Some("ahash")), "ahash");
    assert_eq!(select_algo(Some("mhash")), "mhash");
    assert_eq!(select_algo(Some("phash")), "phash");
    assert_eq!(select_algo(Some("whash")), "whash");

    let result = std::panic::catch_unwind(|| select_algo(Some("unsupported")));
    assert!(result.is_err(), "Expected panic for unsupported algorithm");
}