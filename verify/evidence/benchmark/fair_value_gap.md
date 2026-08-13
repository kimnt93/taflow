# FairValueGap benchmark (`smartmoneyconcepts.smc.fvg` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.128 | 7.80M | 0.111 | 8.99M | 3.084 | 24.06× | 27.74× |
| 10,000 | 1.008 | 9.92M | 0.975 | 10.26M | 8.699 | 8.63× | 8.92× |
| 100,000 | 9.873 | 10.13M | 9.893 | 10.11M | 69.936 | 7.08× | 7.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 2.770 | 26.26× |
| 1 | 5 | 0.473 | 14.139 | 29.87× |
| 1 | 10 | 0.682 | 28.103 | 41.24× |
| 10 | 1 | 0.101 | 2.703 | 26.67× |
| 10 | 5 | 0.350 | 14.172 | 40.50× |
| 10 | 10 | 0.647 | 28.803 | 44.49× |
| 100 | 1 | 0.121 | 2.734 | 22.53× |
| 100 | 5 | 0.401 | 14.392 | 35.92× |
| 100 | 10 | 0.696 | 28.257 | 40.62× |
| 1,000 | 1 | 0.206 | 3.358 | 16.30× |
| 1,000 | 5 | 0.465 | 18.266 | 39.31× |
| 1,000 | 10 | 0.736 | 36.438 | 49.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
