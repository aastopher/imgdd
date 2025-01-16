# Usage

## Installation
```bash
pip install imgdd
```

## Examples

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
