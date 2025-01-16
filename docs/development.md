# Development

## Running Benchmarks

### Rust Benchmarks
Run rust benchmarks, from the project root:
```bash
cargo bench --features benchmarks
```

### Python (integration) Benchmarks

Navigate to `benches` directory:
    
```bash
cd benches
```
    
Install dependencies:
    
```bash
pip install -r requirements.txt
```
    
Run the benchmark script:
    
```bash
python py_bench.py
```
    

## Running Tests

### Local Tests

Run local tests, from the project root:

```bash
cargo test --features testing
```
