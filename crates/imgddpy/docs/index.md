[![imgdd crate](https://img.shields.io/crates/v/imgdd?label=imgdd)](https://crates.io/crates/imgdd)
[![imgddcore crate](https://img.shields.io/crates/v/imgddcore?label=imgddcore)](https://crates.io/crates/imgddcore)
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