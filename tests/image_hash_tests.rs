use imgdd::image_hash::ImageHash;
use image::open;

#[test]
fn test_dhash() {
    let img_path = "./imgs/test/apple_pie/21063.jpg";
    let image = open(img_path).expect("Failed to open image");
    let hash = ImageHash::dhash(&image).expect("Failed to compute dhash");
    assert!(hash.hash > 0, "Hash should be non-zero");
}

#[test]
fn test_hamming_distance() {
    let hash1 = ImageHash { hash: 0b101010 }; // Binary: 42
    let hash2 = ImageHash { hash: 0b111000 }; // Binary: 56

    // Calculate the Hamming distance manually:
    // XOR of 0b101010 and 0b111000 is 0b010010 (binary representation)
    // The Hamming distance is the number of 1 bits: 2

    let expected_distance = 2; // Update to the correct value
    let computed_distance = hash1.hamming_distance(&hash2);

    assert_eq!(computed_distance, expected_distance, "Hamming distance mismatch");
}

