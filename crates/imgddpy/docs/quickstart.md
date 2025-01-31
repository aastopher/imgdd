# Quickstart

## Installation

```bash
pip install imgdd
```

## Usage

### **Hash**
```python
import imgdd as dd

results = dd.hash(
    path="path/to/images",
    algo="dhash",  # Optional: default = dhash
    filter="triangle",  # Optional: default = triangle
    sort=False # Optional: default = False
)
print(results)
```

### **Dupes**
```python
import imgdd as dd

duplicates = dd.dupes(
    path="path/to/images",
    algo="dhash", # Optional: default = dhash
    filter="triangle", # Optional: default = triangle
    remove=False # Optional: default = False
)
print(duplicates)
```