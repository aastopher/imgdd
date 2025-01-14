[![codecov](https://codecov.io/gh/aastopher/imgdd/graph/badge.svg?token=XZ1O2X04SO)](https://codecov.io/gh/aastopher/imgdd)

# imgdd: Image De-Duplication

`imgdd` is a Rust-based Python library for fast and efficient image de-duplication, leveraging perceptual hashing algorithms to identify duplicate or visually similar images in a directory.

## Features
- **Multiple Hashing Algorithms**: Supports `aHash`, `bHash`, `dHash`, `mHash`, `pHash`, and `wHash`.
- **Filter Types**: Supports `Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, and `Lanczos3`.
- **Collect Duplicates**: Harness rusts performance to have imgdd quickly collect duplicate hash pairs and optionally remove the files.
- **Simplicity**: Simple API with robust performance.

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

## Why imgdd?
`imgdd` combine the performance of Rust with the accessability of Python to handle image deduplication efficiently, making it ideal for large datasets.

## License
Licensed under the GNU GPLv3. 

For more details, see the [LICENSE](./LICENSE).