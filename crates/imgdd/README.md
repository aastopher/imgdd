[![codecov](https://codecov.io/gh/aastopher/imgdd/graph/badge.svg?token=XZ1O2X04SO)](https://codecov.io/gh/aastopher/imgdd)
[![Documentation Status](https://img.shields.io/badge/docs-online-brightgreen)](https://aastopher.github.io/imgdd/)
[![DeepSource](https://app.deepsource.com/gh/aastopher/imgdd.svg/?label=active+issues&show_trend=true&token=IiuhCO6n1pK-GAJ800k6Z_9t)](https://app.deepsource.com/gh/aastopher/imgdd/)

# imgdd: Image DeDuplication

`imgdd` is a performance-first perceptual hashing library that combines Rust's speed with Python's accessibility, making it perfect for handling large datasets. Designed to quickly process nested folder structures, commonly found in image datasets.

## Features
- **Multiple Hashing Algorithms**: Supports `aHash`, `dHash`, `mHash`, `pHash`, `wHash`.
- **Multiple Filter Types**: Supports `Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`.
- **Identify Duplicates**: Quickly identify duplicate hash pairs.
- **Simplicity**: Simple interface, robust performance.

## Why imgdd?

`imgdd` has been inspired by [imagehash](https://github.com/JohannesBuchner/imagehash) and aims to be a lightning-fast replacement with additional features. To ensure enhanced performance, `imgdd` has been benchmarked against `imagehash`. In Python, **imgdd consistently outperforms imagehash by ~60%–95%**, demonstrating a significant reduction in hashing time per image.

---

# Quick Start

## Installation

```bash
pip install imgdd
```

## Usage Examples

### Hash Images

```rust
use imgdd::*;
use std::path::PathBuf;

let result = hash(
    PathBuf::from("path/to/images"),
    Some("Triangle"), // Optional: default = "Triangle"
    Some("dHash"),   // Optional: default = "dHash"
    Some(false),     // Optional: default = false
);
println!("{:#?}", result);
```

### Find Duplicates

```rust
use imgdd::*;
use std::path::PathBuf;

let result = dupes(
    PathBuf::from("path/to/images"),
    Some("Triangle"), // Optional: default = "Triangle"
    Some("dHash"),   // Optional: default = "dHash"
    false,
);
println!("{:#?}", result);
```(duplicates)
```

## Supported Algorithms
- **aHash**: Average Hash
- **mHash**: Median Hash
- **dHash**: Difference Hash
- **pHash**: Perceptual Hash
- **wHash**: Wavelet Hash

## Supported Filters
- `Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`

## Contributing
Contributions are always welcome! 🚀  

Found a bug or have a question? Open a GitHub issue. Pull requests for new features or fixes are encouraged!

## Similar projects
- https://github.com/JohannesBuchner/imagehash
- https://github.com/commonsmachinery/blockhash-python
- https://github.com/acoomans/instagram-filters
- https://pippy360.github.io/transformationInvariantImageSearch/
- https://www.phash.org/
- https://pypi.org/project/dhash/
- https://github.com/thorn-oss/perception (based on imagehash code, depends on opencv)
- https://docs.opencv.org/3.4/d4/d93/group__img__hash.html
