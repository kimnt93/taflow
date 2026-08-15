# RollingStandardError benchmark (`StandardError` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 31.93M | 0.029 | 34.00M | 0.183 | 5.86× | 6.24× |
| 10,000 | 0.300 | 33.33M | 0.296 | 33.82M | 0.732 | 2.44× | 2.47× |
| 100,000 | 2.947 | 33.93M | 2.914 | 34.31M | 5.507 | 1.87× | 1.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.059 | 0.257 | 4.35× |
| 1 | 5 | 0.279 | 1.110 | 3.98× |
| 1 | 10 | 0.424 | 2.213 | 5.21× |
| 10 | 1 | 0.046 | 0.212 | 4.55× |
| 10 | 5 | 0.190 | 1.199 | 6.31× |
| 10 | 10 | 0.395 | 2.259 | 5.72× |
| 100 | 1 | 0.050 | 0.205 | 4.13× |
| 100 | 5 | 0.205 | 1.204 | 5.86× |
| 100 | 10 | 0.445 | 2.315 | 5.20× |
| 1,000 | 1 | 0.078 | 0.265 | 3.41× |
| 1,000 | 5 | 0.208 | 1.535 | 7.37× |
| 1,000 | 10 | 0.474 | 2.724 | 5.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
