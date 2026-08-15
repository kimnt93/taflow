# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 42.92M | 0.021 | 47.75M | 0.038 | 1.64× | 1.82× |
| 10,000 | 0.285 | 35.03M | 0.282 | 35.47M | 0.119 | 0.42× | 0.42× |
| 100,000 | 2.870 | 34.85M | 2.745 | 36.43M | 0.824 | 0.29× | 0.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.106 | 1.10× |
| 1 | 5 | 0.310 | 0.495 | 1.60× |
| 1 | 10 | 0.481 | 0.985 | 2.05× |
| 10 | 1 | 0.045 | 0.094 | 2.11× |
| 10 | 5 | 0.204 | 0.452 | 2.22× |
| 10 | 10 | 0.365 | 1.003 | 2.74× |
| 100 | 1 | 0.050 | 0.094 | 1.89× |
| 100 | 5 | 0.210 | 0.442 | 2.11× |
| 100 | 10 | 0.414 | 0.925 | 2.24× |
| 1,000 | 1 | 0.077 | 0.100 | 1.30× |
| 1,000 | 5 | 0.227 | 0.547 | 2.41× |
| 1,000 | 10 | 0.440 | 1.000 | 2.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
