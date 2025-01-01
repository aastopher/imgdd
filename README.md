[![codecov](https://codecov.io/gh/aastopher/imgdd/graph/badge.svg?token=XZ1O2X04SO)](https://codecov.io/gh/aastopher/imgdd)

# imgdd: Image Directory Deduplication

----

`imgdd` is a Rust-based command-line interface (CLI) tool for efficiently detecting and removing visually similar or duplicate images in a directory. Using perceptual hashing and Hamming distance, `imgdd` compares image similarity based on visual content rather than file size or metadata.

## Features

- **Perceptual Hashing**: Uses the `dhash` algorithm to generate hashes based on image content.
- **Customizable Hamming Distance**: Configure the sensitivity of duplicate detection with a distance threshold.

## Usage

Run `imgdd` with the desired options:

```bash
imgdd [OPTIONS]
```

### Options

- **`-p, --path <PATH>`**:  
    Specify the directory containing images to process. Defaults to the current directory if not provided.
    
- **`-t, --threshold <THRESHOLD>`**:  
    Set the Hamming distance threshold for duplicate detection. Images with a hash distance less than or equal to this threshold will be considered duplicates. If not provided, only exact matches are detected.
    
- **`-v, --verbose`**:  
    Enable verbose output for debugging. Use `-vv` for maximum verbosity.
    
- **`-h, --help`**:  
    Show help information.
    
- **`-V, --version`**:  
    Show the application version.
    

### Examples

1. **Detect duplicates in a directory**:
```bash
imgdd -p ./path/to/images
```

2. **Detect duplicates with a Hamming distance threshold of 5**:
```bash
imgdd -p ./path/to/images -t 5
```

3. **Enable verbose output**:
```bash
imgdd -v -p ./path/to/images
```