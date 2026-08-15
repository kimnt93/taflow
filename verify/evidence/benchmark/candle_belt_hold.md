# CandleBeltHold benchmark (`CDLBELTHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.37M | 0.009 | 107.58M | 0.037 | 2.85× | 3.96× |
| 10,000 | 0.136 | 73.55M | 0.135 | 74.07M | 0.126 | 0.92× | 0.93× |
| 100,000 | 1.477 | 67.69M | 1.478 | 67.66M | 1.033 | 0.70× | 0.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.131 | 1.55× |
| 1 | 5 | 0.290 | 0.447 | 1.54× |
| 1 | 10 | 0.386 | 0.931 | 2.41× |
| 10 | 1 | 0.391 | 0.095 | 0.24× |
| 10 | 5 | 0.190 | 0.470 | 2.47× |
| 10 | 10 | 0.381 | 0.871 | 2.29× |
| 100 | 1 | 0.042 | 0.100 | 2.38× |
| 100 | 5 | 0.185 | 0.420 | 2.27× |
| 100 | 10 | 0.393 | 0.946 | 2.41× |
| 1,000 | 1 | 0.058 | 0.101 | 1.73× |
| 1,000 | 5 | 0.200 | 0.472 | 2.36× |
| 1,000 | 10 | 0.407 | 1.064 | 2.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
