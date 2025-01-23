[![codecov](https://codecov.io/gh/aastopher/imgdd/graph/badge.svg?token=XZ1O2X04SO)](https://codecov.io/gh/aastopher/imgdd)
[![DeepSource](https://app.deepsource.com/gh/aastopher/imgdd.svg/?label=active+issues&show_trend=true&token=IiuhCO6n1pK-GAJ800k6Z_9t)](https://app.deepsource.com/gh/aastopher/imgdd/)

# imgdd: Image DeDuplication

`imgdd` is a performance-first perceptual hashing library that combines Rust's speed with Python's accessibility, making it perfect for handling large datasets. Designed to quickly process nested folder structures, commonly found in image datasets.

## Features
- **Multiple Hashing Algorithms**: Supports `aHash`, `mHash`, `dHash`, `pHash`, `wHash`.
- **Multiple Filter Types**: Supports `Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`.
- **Identify Duplicates**: Harness Rust's performance to quickly identify duplicate hash pairs.
- **Simplicity**: Simple interface with robust performance.

## Why imgdd?
`imgdd` has been inspired by [imagehash](https://github.com/JohannesBuchner/imagehash) and aims to be a lightning fast 1:1 replacement with additional features. To ensure enhanced performance `imgdd` has been benchmarked against `imagehash`.

## Planned Features
- **Expanded Algorithm Support**: Add full support for additional perceptual hashing algorithms.
- **Distance Metrics**: Output comprehensive distance metrics to evaluate algorithm and downsampling robustness.
