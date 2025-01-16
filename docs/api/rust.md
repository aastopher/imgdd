# Rust API Reference

`imgdd` is primarily exposed via its Python bindings, but it leverages the following Rust features internally:

## Dedupe Module
Handles hash collection, sorting, and duplicate identification.

### Key Functions
- `collect_hashes`
- `sort_hashes`
- `find_duplicates`

## Hashing Module
Implements various perceptual hashing algorithms.

### Key Functions
- `dhash`