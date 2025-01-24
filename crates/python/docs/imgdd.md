# imgdd: Image DeDuplication

::: imgdd

## Supported Hashing Algorithms

- **aHash (Average Hash):**
    - Calculates average pixel value and compares each pixel to the average.
    - Simple and fast to compute.
    - Suitable for detecting overall image similarity.
  
- **mHash (Median Hash):**
    - Uses the median brightness for more robustness to lighting changes.
    - Suitable for images with varying brightness or exposure levels.
  
- **dHash (Difference Hash):**
    - Encodes relative changes between adjacent pixels.
    - Resistant to small transformations like cropping or rotation.
  
- **pHash (Perceptual Hash):**
    - Analyzes the frequency domain using Discrete Cosine Transform (DCT).
    - Focuses on low-frequency components, which are less affected by resizing or compression.

- **wHash (Wavelet Hash):**
    - Uses Haar wavelet transformations to capture image features.
    - Robust against scaling, rotation, and noise.
