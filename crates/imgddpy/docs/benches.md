# Benchmarks

This section highlights the performance benchmarks for the hashing algorithms provided by [imgdd](https://github.com/aastopher/imgdd) compared to the [imagehash](https://github.com/JohannesBuchner/imagehash) library. The following benchmarks demonstrate significant speed improvements across supported algorithms.

## CPU Details

- **Architecture**: x86_64
- **CPU**: Intel(R) Core(TM) i5-8365U CPU @ 1.60GHz
    - **Cores**: 4 (8 Threads)
    - **Max Frequency**: 4.1 GHz
    - **Base Frequency**: 1.6 GHz

## Rust Benchmarks

Below is a snapshot of local bare metal benchmarks taken on using Criterion directly on the imgddcore Rust crate, based on the hardware details above.

|Algorithm|Time (ms)|Measurements|
|---|---|---|
|aHash|0.815|100|
|mHash|1.369|100|
|dHash|0.541|100|
|pHash|23.709|100|
|wHash|3.345|100|

---

## Python Integration Benchmarks

The table below compares the local performance of [imgdd](https://github.com/aastopher/imgdd) with the [imagehash](https://github.com/JohannesBuchner/imagehash) library, based on the hardware details above.

### dHash

|Metric|imgdd (ms)|imagehash (ms)|Improvement (%)|
|---|---|---|---|
|Min Time|1.788|5.098|64.92|
|Max Time|2.792|7.684|63.67|
|Avg Time|1.942|5.645|65.59|
|Median Time|1.888|5.554|66.02|

### aHash

|Metric|imgdd (ms)|imagehash (ms)|Improvement (%)|
|---|---|---|---|
|Min Time|1.683|5.666|70.29|
|Max Time|3.207|15.403|79.18|
|Avg Time|2.055|8.346|75.38|
|Median Time|2.043|7.683|73.41|

### pHash

|Metric|imgdd (ms)|imagehash (ms)|Improvement (%)|
|---|---|---|---|
|Min Time|1.798|5.726|68.60|
|Max Time|4.063|20.099|79.78|
|Avg Time|2.361|7.896|70.10|
|Median Time|2.138|7.196|70.29|

### wHash

|Metric|imgdd (ms)|imagehash (ms)|Improvement (%)|
|---|---|---|---|
|Min Time|1.750|42.418|95.87|
|Max Time|4.422|97.446|95.46|
|Avg Time|2.192|62.656|96.50|
|Median Time|1.978|60.397|96.72|

---

## Summary

- In Python, **imgdd consistently outperforms imagehash by ~60%–95%**, demonstrating a significant reduction in hashing time per image.
- imgddcore rust benchmarks achieve **sub-1 ms performance** for dHash and aHash, while maintaining excellent speeds across all algorithms.
