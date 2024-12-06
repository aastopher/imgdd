# imgdd: Image Directory Deduplication CLI

`imgdd` is a Rust-based command-line interface (CLI) tool for efficiently detecting and removing visually similar or duplicate images in a directory. Using perceptual hashing and Hamming distance, `imgdd` compares image similarity based on visual content rather than file size or metadata.

## Features

- **Perceptual Hashing**: Uses the `dhash` algorithm to generate hashes based on image content.
- **Customizable Hamming Distance**: Configure the sensitivity of duplicate detection with a distance threshold.

## Usage

Run `imgdd` with the desired options:

```sh
imgdd [OPTIONS]
```

### Options:

- **`-p`, `--path <PATH>`**  
    Specify the directory containing images to process. Defaults to the current directory if not provided.
    
- **`-d`, `--distance <DISTANCE>`**  
    Set the Hamming distance threshold for duplicate detection. Images with a hash distance less than or equal to this threshold will be considered duplicates. Default: `12`.
    
- **`-v`, `--verbose`**  
    Enable verbose output for debugging. Use `-vv` for maximum verbosity.
    
- **`-h`, `--help`**  
    Show help information.
    
- **`-V`, `--version`**  
    Show the application version.
    

## How It Works

1. **Hash Calculation**: Images are processed to compute a perceptual hash using the `dhash` algorithm.
2. **Hamming Distance**: Hashes are compared using Hamming distance to measure visual similarity.
3. **Duplicate Removal**: Images deemed duplicates (within the threshold) are deleted, retaining only one unique version.