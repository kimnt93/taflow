# ExponentiallyWeightedVariance benchmark (`ewm variance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.96M | 0.006 | 160.20M | 1.281 | 183.08× | 205.15× |
| 10,000 | 0.046 | 215.46M | 0.043 | 234.66M | 12.432 | 267.85× | 291.72× |
| 100,000 | 0.406 | 246.45M | 0.414 | 241.84M | 122.143 | 301.02× | 295.39× |
| 1,000,000 | 4.222 | 236.83M | 3.874 | 258.13M | 1266.102 | 299.85× | 326.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.138 | 1.20× |
| 1 | 5 | 0.379 | 0.411 | 1.09× |
| 1 | 10 | 0.443 | 0.809 | 1.83× |
| 10 | 1 | 0.046 | 0.101 | 2.17× |
| 10 | 5 | 0.214 | 0.464 | 2.17× |
| 10 | 10 | 0.430 | 0.970 | 2.25× |
| 100 | 1 | 0.048 | 0.212 | 4.42× |
| 100 | 5 | 0.229 | 1.042 | 4.55× |
| 100 | 10 | 0.472 | 2.188 | 4.63× |
| 1,000 | 1 | 0.058 | 1.342 | 23.20× |
| 1,000 | 5 | 0.234 | 6.819 | 29.14× |
| 1,000 | 10 | 0.502 | 13.749 | 27.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
