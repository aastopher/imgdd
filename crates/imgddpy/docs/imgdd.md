# imgdd: Image DeDuplication

::: imgdd

## Supported Hashing Algorithms

- **aHash (Average Hash):**
    - Calculates average pixel value and compares each pixel to the average.
    - Simple and fast to compute.
    - Suitable for detecting overall image similarity.
  
- **mHash (Median Hash):**
    - Uses median brightness for more robustness to lighting changes.
    - Suitable for images with varying brightness or exposure levels.
  
- **dHash (Difference Hash):**
    - Encodes relative changes between adjacent pixels.
    - Resistant to small transformations like cropping and rotation.
  
- **pHash (Perceptual Hash):**
    - Analyzes frequency domain using Discrete Cosine Transform (DCT).
    - Focuses on low-frequency components, which are less affected by resizing and compression.

- **wHash (Wavelet Hash):**
    - Uses Haar wavelet transformations to capture image features.
    - Robust against scaling, rotation, and noise.
