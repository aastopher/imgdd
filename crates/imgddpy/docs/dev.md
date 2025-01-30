# Development

## Building

### Python package

Build local python wheel file, from the project root
```bash
maturin build --release --manifest-path crates/imgddpy/Cargo.toml
```

### Rust Crates

Build all rust crates, from the project root
```bash
cargo build
```

## Running Tests

### Python (integration) Tests

Navigate to `imgddpy` directory
```bash
cd crates/imgddpy/
```

Install dependencies
```bash
pip install .[test]
```

Run tests
```bash
pytest
```

### Rust Tests

Run local tests, from the project root
```bash
cargo test --features testing
```

## Running Benchmarks

### Rust Benchmarks
Run rust benchmarks, from the project root
```bash
cargo bench --features benchmark
```

### Python Benchmarks
Navigate to `imgddpy` directory
```bash
cd crates/imgddpy/
```

Install dependencies
```bash
pip install .[test]
```

Run python benchmarks, from the project root
```bash
pytest -m benchmark --codspeed -v
```
    
## Imgdd vs Imagehash Comparison

Navigate to `comparison` directory
```bash
cd crates/imgddpy/comparison/
```
    
Install dependencies
```bash
pip install -r requirements.txt
```
    
Run the compare script
```bash
python compare.py
```

## Docs

### Python Docs

Navigate to `imgddpy` directory
```bash
cd crates/imgddpy/
```
    
Install dependencies
```bash
pip install .[dev]
```

Build docs
```bash
mkdocs build
```

Serve docs
```bash
mkdocs serve
```

### Rust Docs

Build docs
```bash
cargo doc --no-deps
```