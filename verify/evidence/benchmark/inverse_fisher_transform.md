# InverseFisherTransform benchmark (`InverseFisherTransform` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.86M | 0.005 | 182.63M | 0.167 | 24.68× | 30.48× |
| 10,000 | 0.035 | 282.03M | 0.034 | 294.90M | 0.481 | 13.56× | 14.18× |
| 100,000 | 0.322 | 310.85M | 0.300 | 333.75M | 3.634 | 11.30× | 12.13× |
| 1,000,000 | 3.791 | 263.80M | 3.080 | 324.70M | 33.072 | 8.72× | 10.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.174 | 0.265 | 1.52× |
| 1 | 5 | 0.248 | 1.228 | 4.95× |
| 1 | 10 | 0.505 | 2.574 | 5.09× |
| 10 | 1 | 0.068 | 0.259 | 3.82× |
| 10 | 5 | 0.272 | 1.386 | 5.10× |
| 10 | 10 | 0.554 | 2.525 | 4.56× |
| 100 | 1 | 0.054 | 0.210 | 3.87× |
| 100 | 5 | 0.263 | 1.409 | 5.37× |
| 100 | 10 | 0.509 | 2.436 | 4.79× |
| 1,000 | 1 | 0.058 | 0.250 | 4.32× |
| 1,000 | 5 | 0.266 | 1.532 | 5.75× |
| 1,000 | 10 | 0.567 | 2.849 | 5.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
