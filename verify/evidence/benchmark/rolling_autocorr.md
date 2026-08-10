# RollingAutocorr benchmark (`Autocorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.060 | 16.73M | 0.064 | 15.62M | 0.253 | 4.24× | 3.96× |
| 10,000 | 0.576 | 17.36M | 0.576 | 17.36M | 1.050 | 1.82× | 1.82× |
| 100,000 | 5.638 | 17.74M | 5.849 | 17.10M | 9.357 | 1.66× | 1.60× |
| 1,000,000 | 56.921 | 17.57M | 57.199 | 17.48M | 107.524 | 1.89× | 1.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.154 | 0.298 | 1.93× |
| 1 | 5 | 0.281 | 1.445 | 5.14× |
| 1 | 10 | 0.524 | 2.908 | 5.55× |
| 10 | 1 | 0.054 | 0.237 | 4.39× |
| 10 | 5 | 0.253 | 1.461 | 5.78× |
| 10 | 10 | 0.488 | 2.483 | 5.09× |
| 100 | 1 | 0.062 | 0.252 | 4.06× |
| 100 | 5 | 0.235 | 1.510 | 6.42× |
| 100 | 10 | 0.503 | 2.780 | 5.53× |
| 1,000 | 1 | 0.122 | 0.334 | 2.73× |
| 1,000 | 5 | 0.275 | 1.960 | 7.12× |
| 1,000 | 10 | 0.539 | 3.435 | 6.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
