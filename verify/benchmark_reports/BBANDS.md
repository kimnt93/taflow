# BollingerBands benchmark (`BBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.67M | 0.006 | 170.82M | 0.053 | 6.62× | 9.00× |
| 10,000 | 0.045 | 223.76M | 0.037 | 269.09M | 0.095 | 2.14× | 2.57× |
| 100,000 | 0.452 | 221.28M | 0.395 | 253.06M | 0.552 | 1.22× | 1.40× |
| 1,000,000 | 15.036 | 66.51M | 7.388 | 135.35M | 7.433 | 0.49× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.133 | 1.42× |
| 1 | 5 | 0.316 | 0.653 | 2.07× |
| 1 | 10 | 0.487 | 1.228 | 2.52× |
| 10 | 1 | 0.054 | 0.121 | 2.24× |
| 10 | 5 | 0.232 | 0.551 | 2.38× |
| 10 | 10 | 0.442 | 1.115 | 2.52× |
| 100 | 1 | 0.052 | 0.126 | 2.41× |
| 100 | 5 | 0.229 | 0.554 | 2.42× |
| 100 | 10 | 0.466 | 1.136 | 2.44× |
| 1,000 | 1 | 0.052 | 0.114 | 2.20× |
| 1,000 | 5 | 0.250 | 0.602 | 2.40× |
| 1,000 | 10 | 0.483 | 1.161 | 2.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
