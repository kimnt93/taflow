# ExponentiallyWeightedCorrelation benchmark (`ewm correlation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.20M | 0.010 | 100.09M | 1.383 | 124.73× | 138.42× |
| 10,000 | 0.069 | 144.24M | 0.063 | 157.80M | 13.101 | 188.97× | 206.73× |
| 100,000 | 0.601 | 166.46M | 0.575 | 173.88M | 129.454 | 215.49× | 225.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.143 | 1.65× |
| 1 | 5 | 0.305 | 0.574 | 1.88× |
| 1 | 10 | 0.509 | 1.010 | 1.99× |
| 10 | 1 | 0.059 | 0.118 | 2.00× |
| 10 | 5 | 0.256 | 0.582 | 2.27× |
| 10 | 10 | 0.486 | 1.153 | 2.37× |
| 100 | 1 | 0.055 | 0.228 | 4.14× |
| 100 | 5 | 0.247 | 1.130 | 4.58× |
| 100 | 10 | 0.527 | 2.283 | 4.33× |
| 1,000 | 1 | 0.058 | 1.396 | 24.01× |
| 1,000 | 5 | 0.239 | 7.072 | 29.57× |
| 1,000 | 10 | 0.563 | 14.525 | 25.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
