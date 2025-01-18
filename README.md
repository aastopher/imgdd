[![codecov](https://codecov.io/gh/aastopher/imgdd/graph/badge.svg?token=XZ1O2X04SO)](https://codecov.io/gh/aastopher/imgdd)
[![DeepSource](https://app.deepsource.com/gh/aastopher/imgdd.svg/?label=active+issues&show_trend=true&token=IiuhCO6n1pK-GAJ800k6Z_9t)](https://app.deepsource.com/gh/aastopher/imgdd/)

# imgdd: Image DeDuplication

`imgdd` is a Rust-based Python library for fast and efficient image deduplication, leveraging perceptual hashing algorithms to identify duplicate or visually similar images in a directory.

## Features
- **Multiple Hashing Algorithms**: Supports `aHash`, `bHash`, `dHash`, `mHash`, `pHash`, and `wHash`.
- **Multiple Filter Types**: Supports `Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, and `Lanczos3`.
- **Identify Duplicates**: Harness Rust's performance to quickly identify duplicate hash pairs and optionally remove the files.
- **Simplicity**: Simple interface with robust performance.

## Why imgdd?
`imgdd` combines the performance of Rust with the accessibility of Python to handle image deduplication efficiently, making it ideal for large datasets.

---

# Quick Start

## Installation
```bash
pip install imgdd
```

## Usage Examples

### Hash Images
```python
import imgdd as dd

results = dd.hash(
    path="path/to/images",
    algo="dhash",  # Optional: default = dhash
    filter="nearest"  # Optional: default = nearest
)
print(results)
```

### Find Duplicates
```python
duplicates = dd.dupes(
    path="path/to/images",
    algo="dhash", # Optional: default = dhash
    filter="gaussian", # Optional: default = nearest
    remove=True # Optional: default = False
)
print(duplicates)
```

## Supported Algorithms
- **aHash**: Average Hash (WIP)
- **bHash**: Block Hash (WIP)
- **dHash**: Difference Hash
- **mHash**: Median Hash (WIP)
- **pHash**: Perceptual Hash (WIP)
- **wHash**: Wavelet Hash (WIP)

## Supported Filters
- `Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`